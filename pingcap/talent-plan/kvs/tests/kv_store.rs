use kvs::thread_pool::RayonThreadPool;
use kvs::{KvStore, KvsEngine, Result};
use tempfile::TempDir;
use walkdir::WalkDir;

// Should get previously stored value
#[tokio::test]
async fn get_stored_value() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let store = KvStore::<RayonThreadPool>::open(temp_dir.path(), 1)?;

    store.set("key1".to_owned(), "value1".to_owned()).await?;
    store.set("key2".to_owned(), "value2".to_owned()).await?;

    assert_eq!(
        store.get("key1".to_owned()).await?,
        Some("value1".to_owned())
    );
    assert_eq!(
        store.get("key2".to_owned()).await?,
        Some("value2".to_owned())
    );

    // Open from disk again and check persistent data
    drop(store);
    let store = KvStore::<RayonThreadPool>::open(temp_dir.path(), 1)?;
    assert_eq!(
        store.get("key1".to_owned()).await?,
        Some("value1".to_owned())
    );
    assert_eq!(
        store.get("key2".to_owned()).await?,
        Some("value2".to_owned())
    );

    Ok(())
}

// Should overwrite existent value
#[tokio::test]
async fn overwrite_value() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let store = KvStore::<RayonThreadPool>::open(temp_dir.path(), 1)?;

    store.set("key1".to_owned(), "value1".to_owned()).await?;
    assert_eq!(
        store.get("key1".to_owned()).await?,
        Some("value1".to_owned())
    );
    store.set("key1".to_owned(), "value2".to_owned()).await?;
    assert_eq!(
        store.get("key1".to_owned()).await?,
        Some("value2".to_owned())
    );

    // Open from disk again and check persistent data
    drop(store);
    let store = KvStore::<RayonThreadPool>::open(temp_dir.path(), 1)?;
    assert_eq!(
        store.get("key1".to_owned()).await?,
        Some("value2".to_owned())
    );
    store.set("key1".to_owned(), "value3".to_owned()).await?;
    assert_eq!(
        store.get("key1".to_owned()).await?,
        Some("value3".to_owned())
    );

    Ok(())
}

// Should get `None` when getting a non-existent key
#[tokio::test]
async fn get_non_existent_value() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let store = KvStore::<RayonThreadPool>::open(temp_dir.path(), 1)?;

    store.set("key1".to_owned(), "value1".to_owned()).await?;
    assert_eq!(store.get("key2".to_owned()).await?, None);

    // Open from disk again and check persistent data
    drop(store);
    let store = KvStore::<RayonThreadPool>::open(temp_dir.path(), 1)?;
    assert_eq!(store.get("key2".to_owned()).await?, None);

    Ok(())
}

#[tokio::test]
async fn remove_non_existent_key() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let store = KvStore::<RayonThreadPool>::open(temp_dir.path(), 1)?;
    assert!(store.remove("key1".to_owned()).await.is_err());
    Ok(())
}

#[tokio::test]
async fn remove_key() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let store = KvStore::<RayonThreadPool>::open(temp_dir.path(), 1)?;
    store.set("key1".to_owned(), "value1".to_owned()).await?;
    assert!(store.remove("key1".to_owned()).await.is_ok());
    assert_eq!(store.get("key1".to_owned()).await?, None);
    Ok(())
}

// Insert data until total size of the directory decreases.
// Test data correctness after compaction.
#[tokio::test]
async fn compaction() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let store = KvStore::<RayonThreadPool>::open(temp_dir.path(), 1)?;

    let dir_size = || {
        let entries = WalkDir::new(temp_dir.path()).into_iter();
        let len: walkdir::Result<u64> = entries
            .map(|res| {
                res.and_then(|entry| entry.metadata())
                    .map(|metadata| metadata.len())
            })
            .sum();
        len.expect("fail to get directory size")
    };

    let mut current_size = dir_size();
    for iter in 0..1000 {
        for key_id in 0..1000 {
            let key = format!("key{}", key_id);
            let value = format!("{}", iter);
            store.set(key, value).await?;
        }

        let new_size = dir_size();
        if new_size > current_size {
            current_size = new_size;
            continue;
        }
        // Compaction triggered

        drop(store);
        // reopen and check content
        let store = KvStore::<RayonThreadPool>::open(temp_dir.path(), 1)?;
        for key_id in 0..1000 {
            let key = format!("key{}", key_id);
            assert_eq!(store.get(key).await?, Some(format!("{}", iter)));
        }
        return Ok(());
    }

    panic!("No compaction detected");
}

#[tokio::test]
async fn concurrent_set() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");

    // concurrent set in 8 threads
    let store = KvStore::<RayonThreadPool>::open(temp_dir.path(), 8)?;

    let mut handles = vec![];
    for i in 0..10000 {
        let in_store = store.clone();
        handles.push(tokio::spawn(async move {
            in_store
                .set(format!("key{}", i), format!("value{}", i)).await
        }));
    }
    futures::future::join_all(handles).await;
    // We only check concurrent set in this test, so we check sequentially here
    let store = KvStore::<RayonThreadPool>::open(temp_dir.path(), 1)?;
    for i in 0..10000 {
        assert_eq!(
            store.get(format!("key{}", i)).await?,
            Some(format!("value{}", i))
        );
    }

    Ok(())
}

#[tokio::test]
async fn concurrent_get() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let store = KvStore::<RayonThreadPool>::open(temp_dir.path(), 8)?;
    // We only check concurrent get in this test, so we set sequentially here
    for i in 0..100 {
        store
            .set(format!("key{}", i), format!("value{}", i))
            .await
            .unwrap();
    }

    let mut handles = vec![];
    for thread_id in 0..100 {
        for i in 0..100 {
            let key_id = (i + thread_id) % 100;
            let in_store = store.clone();
            handles.push(tokio::spawn(  async move{
                in_store
                    .get(format!("key{}", key_id)).await
                    .map(move |res| {
                        assert_eq!(res, Some(format!("value{}", key_id)));
                    })
            }));
        }
    }
    futures::future::join_all(handles).await;


    // reload from disk and test again
    let store = KvStore::<RayonThreadPool>::open(temp_dir.path(), 8)?;

    let mut handles = vec![];
    for thread_id in 0..100 {
        for i in 0..100 {
            let key_id = (i + thread_id) % 100;
            let in_store = store.clone();
            handles.push(tokio::spawn( async move {
                in_store
                    .get(format!("key{}", key_id)).await
                    .map(move |res| {
                        assert_eq!(res, Some(format!("value{}", key_id)));
                    })
            }));
        }
    }
    futures::future::join_all(handles).await;

    Ok(())
}
