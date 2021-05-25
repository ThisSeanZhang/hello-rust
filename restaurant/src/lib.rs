mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }



}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    self::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    // 选择黑麦面包作为夏季早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 修改我们想要的面包类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // 接下来的这一行无法通过编译，我们不能看到或更换随着食物附带的季节性水果
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
fn serve_order() {}
mod back_of_house {
    fn cook_order() {}
    fn fix_incorrect_order() {
        cook_order();
        // super是所处模块的父模块
        super::serve_order();
    }

    /**
    * 枚举公开时
    */
    pub enum Appetizer {
        Soup,
        Salad,
    }

    /**
    * 结构体在公开时 是可以决定仅公开部分属性
    */
    pub struct Breakfast {
        pub toast: String, // 因为是公开字段  所以能够进行修改
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }



}
