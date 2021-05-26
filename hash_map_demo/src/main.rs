use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> =
        teams.iter().zip(initial_scores.iter()).collect();


    /**
    * 那些实现了Copy trait的类型，例如i32，它们的值会被简单
    * 地复制到哈希映射中。而对于String这种持有所有权的值，其值将会
    * 转移且所有权会转移给哈希映射
    */
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // filed_name和field_value从这一刻开始失效，若尝试使用它们则会导致编译错误！

    let field_name = &String::from("Favorite color");
    let field_value = &String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // filed_name和field_value在此处还是有效的

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    if let Some(score) = score {
        println!("{} value is {}", team_name, score);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{} : {}", key, value)
    }
    // 覆盖已有的
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // 如果不存在Red则插入一个
    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // 在已有值的基础上更新值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
