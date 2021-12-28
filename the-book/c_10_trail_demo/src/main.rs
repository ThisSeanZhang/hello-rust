mod trait_demo;
mod trait_demo2;

use crate::trait_demo::*;
use crate::trait_demo2::CCC;
use crate::trait_demo2::ToWhat;
// use crate::trait_demo2;
fn main() {

    println!("Hello, world!");
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());


    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
 hockey team in the NHL."),
    };
    println!("New article available! {}", article.summarize());


    let p = trait_demo2::Pair::new(1, 2);
    p.doCcc();
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 完整形式
// pub fn notify<T: Summary>(item: T) {
//     println!("Breaking news! {}", item.summarize());
// }

// 多个参数
// pub fn notify(item1: impl Summary, item2: impl Summary)
// pub fn notify<T: Summary>(item1: T, item2: T)

// 多种类型的约束
// pub fn notify(item: impl Summary + Display)
// pub fn notify<T: Summary + Display>(item: T)

// 使用where 进行约束
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32
// fn some_function<T, U>(t: T, u: U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {


/// 不仅传入可以使用trail, 返回值同样也可以
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
