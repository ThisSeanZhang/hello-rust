use std::time::Instant;

mod int_list;
mod s_list;

fn main() {
    let mut slist = s_list::SList::new(None);
    let now = Instant::now();
    for i in 0..10_000_000 {
        slist.add_first(i);
    }
    let elapsed_time = now.elapsed();
    println!("use ing timeing: {}", elapsed_time.as_millis());
}
