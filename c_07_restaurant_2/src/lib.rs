mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
// 还能在前部增加pub
use crate::front_of_house::hosting;
// 使用self相对路径引入
// use self::front_of_house::hosting;
// 直接将方法引入 但不建议这样使用
// use crate::front_of_house::hosting::add_to_waitlist;
// 引用结构体\枚举和其他条目 使用完整路径的方式
use std::collections::HashMap;

// 可以使用as对重复的名称进行重命名
use std::fmt::Result;
use std::io::Result as IoResult;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
// fn main() {}

