mod pare_point;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let both_integer = Point { x: 5, y: 10 };
    println!("{:?}", both_integer);
    let both_float = Point { x: 1.0, y: 4.0 };
    println!("p.x = {}", both_float.x());
    println!("distance_from_origin = {}", both_float.distance_from_origin());
    // 因为类型不匹配  所以不能用该方法
    // println!("distance_from_origin = {}", both_integer.distance_from_origin());
    // 以下因为x,y的类型不匹配  将会抛错
    // let integer_and_float = Point { x: 5, y: 4.0 };
    pare_point_main();
}

fn pare_point_main() {
    let p1 = pare_point::Point { x: 5, y: 10.4 };
    let p2 = pare_point::Point { x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// fn largest_char(list: &[char]) -> char {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// 也可为单独的类型设置方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
