fn main() {
    println!("Hello, world!");
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    // 根据第3条规则因为第一个是self 所以返回的生命周期与self相同
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }

    // 此时因为announcement与返回的生命周期(与self相同)不同，所以无法通过编译
    // fn announce_and_return_part2(&self, announcement: &str) -> &str {
    //     println!("Attention please: {}", announcement);
    //     announcement
    // }
    // 手工标注为相同的生命周期就行了
    fn announce_and_return_part3<'c>(&self, announcement: &'c str) -> &'c str {
        println!("Attention please: {}", announcement);
        announcement
    }
}