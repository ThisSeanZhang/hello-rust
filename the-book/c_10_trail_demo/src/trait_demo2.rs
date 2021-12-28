use std::fmt::Display;
pub struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

pub trait CCC {
    fn ccc(&self) {
        println!("ccc in CCC")
    }
}
pub trait ToWhat{
    fn doCcc(&self);
}

impl<T> CCC for Pair<T> {
}

impl <T: CCC> ToWhat for T {
    fn doCcc(&self) {
        println!("do CCC");
        self.ccc()
    }
}
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
// impl<T: Display> ToString for T {
//     // --略--
// }

