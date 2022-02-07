use core::borrow;
use std::{collections::HashMap, ops::Deref};

type Table = HashMap<String, Vec<String>>;

fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(), vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string(), "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(), vec!["Perseus with the head of Medusa".to_string(), "a salt cellar".to_string()]);
    show(&table);
    sort_works(&mut table);
}


fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

#[test]
fn references_as_value() {
    struct Anime { name: &'static str, bechdel_pass: bool };
    let aria = Anime { name: "Aria: The Animation", bechdel_pass: true };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");
    // Equivalent to the above, but with the dereference written out:
    assert_eq!((*anime_ref).name, "Aria: The Animation");

    let mut v = vec![1973, 1968];
    v.sort(); // implicitly borrows a mutable reference to v
    (&mut v).sort(); // equivalent; much uglier
}

#[test]
fn assigning_references() {
    let x = 10;
    let y = 20;
    let mut r = &x;
    let b= true;
    if b { r = &y; }
    assert!(*r == 10 || *r == 20);
}

#[test]
fn references_references() {
    struct Point { x: i32, y: i32 }
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;

    assert_eq!(rrr.y, 729);
}

#[test]
fn comparing_references() {
    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;
    println!("{}", rx);
    println!("{}", rrx);
    assert!(rrx <= rry);
    assert!(rrx == rry);

    assert!(!std::ptr::eq(rx, ry));
}

#[test]
fn borrowing_references_arbitrary() {
    fn factorial(n: usize) -> usize {
        (1..n+1).fold(1, |a, b| a * b)
    }
    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);
       
}

#[test]
fn borrowing_local_variable() {
    // {
    //     let r;
    //     {
    //     let x = 1;
    //     r = &x;
    //     }
    //     assert_eq!(*r, 1); // bad: reads memory `x` used to occupy
    // }
}

#[test]
fn returning_references() {
    // v should have at least one element.
    fn smallest(v: &[i32]) -> &i32 {
        let mut s = &v[0];
        for r in &v[1..] {
        if *r < *s { s = r; }
        }
        s
    }

    let s;
    {
     let parabola = [9, 4, 1, 0, 1, 4, 9];
     s = smallest(&parabola);
    }
    // assert_eq!(*s, 0); // bad: points to element of dropped array
}

#[test]
fn structs_containing_referen() {
    // This does not compile.
// struct S {
//     r: &i32
//    }
//    let s;
//    {
//     let x = 10;
//     s = S { r: &x };
//    }
//    assert_eq!(*s.r, 10); // bad: reads from dropped `x`
   
}

#[test]
fn test_borrowing_part() {
    // let mut w = (107, 109);
    // let r = &w;
    // let r0 = &r.0; // ok: reborrowing shared as shared
    // let m1 = &mut r.1; // error: can't reborrow shared as mutable
}

#[test]
fn test_borrowing_part2() {
    let mut w = (10, 42);
    let r0 = &mut w.0;
    let r1 = &w.1;

    println!("{}, {}", r0, r1);
}

#[test]
fn test_string_asref() {
    fn as_test<A: AsRef<[u8]>>(data: A) {
        println!("{:?}", data.as_ref());
    }
    as_test("aaa".to_string());
}

#[test]
fn borrow_hashmap() {
    struct Foo {
        name: String
    }

    impl std::borrow::Borrow<str> for Foo {
        fn borrow(&self) -> &str {
            &self.name
        }
    }

    fn need_str<A>(data: A)
    where A: std::borrow::Borrow<str> {
        println!("{}", data.borrow());
    }
    need_str(Foo{name: "cccc".to_string()})
}

#[test]
fn test_string() {
    String::from("");
    let mut a = "a";
    let b: String = a.to_owned();
    a = "b";
    println!("{}", a);
    println!("{}", b);
}

#[test]
fn test_trait() {

    #[derive(Debug)]
    struct Node {
        name: String
    }

    impl Deref for Node {
        type Target = String;
        fn deref(&self) -> &Self::Target {
            println!("use deref fn");
            &self.name
        }
    }

    let node = Node { name: "node1".to_string() };
    let ref_node = &node;
    let ref_ref_node = &ref_node;
    println!("print node {:?}", ref_ref_node);
    println!("======");
    println!("{}", ref_ref_node as &String);
    println!("======");
    println!("{:?}", ref_ref_node.deref());
}