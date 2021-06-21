#[cfg(test)]
mod tests {
    use crate::link::List::{Cons, Nil};
    #[test]
    fn test1() {
        let list = Cons(1, Cons(2, Cons(3, Nil)));
    }


}
enum List {
    Cons(i32, List),
    Nil,
}