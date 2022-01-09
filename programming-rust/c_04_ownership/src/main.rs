fn main() {
    
    fn print_padovan() {
        let mut padovan = vec![1,1,1]; // allocated here
        for i in 3..10 {
            let next = padovan[i-3] + padovan[i-2];
            padovan.push(next);
        }
        println!("P(1..10) = {:?}", padovan);
    }

    {
        let point = Box::new((0.625, 0.5)); // point allocated here
        let label = format!("{:?}", point); // label allocated here
        assert_eq!(label, "(0.625, 0.5)");
    } // both dropped here

    struct Person { name: String, birth: i32 }
    let mut composers = Vec::new();
    composers.push(Person { name: "Palestrina".to_string(), birth: 1525 });
    composers.push(Person { name: "Dowland".to_string(), birth: 1563 });
    composers.push(Person { name: "Lully".to_string(), birth: 1632 });
    
    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }

    // let x = vec![10, 20, 30];
    // if c {
    //     f(x); // ... ok to move from x here
    // } else {
    //     g(x); // ... and ok to also move from x here
    // }
    // h(x) // bad: x is uninitialized here if either path uses it

    // let x = vec![10, 20, 30];
    // while f() {
    //     g(x); // bad: x would be moved in first iteration,
    // // uninitialized in second
    // }

    // let mut x = vec![10, 20, 30];
    // while f() {
    //     g(x); // move from x
    //     x = h(); // give x a fresh value
    // }
    // e(x);
    

    // Build a vector of the strings "101", "102", ... "105"
    let mut v = Vec::new();
    for i in 101 .. 106 {
        v.push(i.to_string());
    }
    // Pull out random elements from the vector.
    // let third = v[2];
    // let fifth = v[4];


    // Build a vector of the strings "101", "102", ... "105"
    let mut v = Vec::new();
    for i in 101 .. 106 {
    v.push(i.to_string());
    }
    // 1. Pop a value off the end of the vector:
    let fifth = v.pop().unwrap();
    assert_eq!(fifth, "105");
    // 2. Move a value out of the middle of the vector, and move the last
    // element into its spot:
    let second = v.swap_remove(1);
    assert_eq!(second, "102");
    // 3. Swap in another value for the one we're taking out:
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");
    // Let's see what's left of our vector.
    assert_eq!(v, vec!["101", "104", "substitute"]);
}
