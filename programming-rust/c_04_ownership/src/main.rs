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
}
