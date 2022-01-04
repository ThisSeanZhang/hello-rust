fn main() {

    assert_eq!( 10_i8 as u16, 10_u16); // in range
    assert_eq!( 2525_u16 as i16, 2525_i16); // in range
    assert_eq!( -1_i16 as i32, -1_i32); // sign-extended
    assert_eq!(65535_u16 as i32, 65535_i32); // zero-extended
    // Conversions that are out of range for the destination
    // produce values that are equivalent to the original modulo 2^N,
    // where N is the width of the destination in bits. This
    // is sometimes called "truncation".
    assert_eq!( 1000_i16 as u8, 232_u8);
    assert_eq!(1000 % 256 , 232);
    assert_eq!(65535_u32 as i16, -1_i16);
    assert_eq!(65536_u32 as i16, 0_i16);
    assert_eq!(65537_u32 as i16, 1_i16);
    assert_eq!( -1_i8 as u8, 255_u8);
    assert_eq!( 255_u8 as i8, -1_i8);

    assert_eq!(2u16.pow(4), 16); // exponentiation
    assert_eq!((-4i32).abs(), 4); // absolute value
    assert_eq!(0b101101u8.count_ones(), 4); // population count
    println!("{}", 3_i32.count_ones());

    let mut sum = 0_f64;
    for _ in 0..100 {
        sum += 0.1;
    }
    println!("{}", sum);

    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.); // exactly 5.0, per IEEE
    assert_eq!(-1.01f64.floor(), -1.0);
    assert!((-1. / std::f32::INFINITY).is_sign_negative());

    println!("{}", (2.0_f64).sqrt());
    println!("{}", f64::sqrt(2.0));

    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);


    assert_eq!('*' as i32, 42);
    assert_eq!('*' as u16, 0x2A);
    // assert_eq!('*' as i8, -(i8::MAX - 42)); // U+0CA0 truncated to eight bits, signed


    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
    
    let temp = text.split_at(21);
    let head = temp.0;
    let tail = temp.1;
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    // Arrays
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];
    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    assert!(sieve[211]);
    assert!(!sieve[9876]);

    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);

    // Vectors
    let mut v = vec![2, 3, 5, 7];
    assert_eq!(v.iter().fold(1, |a, b| a * b), 210);

    v.push(11);
    v.push(13);
    assert_eq!(v.iter().fold(1, |a, b| a * b), 30030);

    fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
        vec![0; rows * cols]
    }

    let test = new_pixel_buffer(1,1);
    println!("{:?}", test);

    let mut v = Vec::new();
    v.push("step");
    v.push("on");
    v.push("no");
    v.push("pets");
    assert_eq!(v, vec!["step", "on", "no", "pets"]);

    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);

    // A palindrome!
    let mut v = vec!["a man", "a plan", "a canal", "panama"];
    v.reverse();
    // Reasonable yet disappointing:
    assert_eq!(v, vec!["panama", "a canal", "a plan", "a man"]);


    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);
    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);
    v.push(3);
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 4);


    let mut v = vec![10, 20, 30, 40, 50];
    // Make the element at index 3 be 35.
    v.insert(3, 35);
    assert_eq!(v, [10, 20, 30, 35, 40, 50]);
    // Remove the element at index 1.
    v.remove(1);
    assert_eq!(v, [10, 30, 35, 40, 50]);

    let mut v = vec!["carmen", "miranda"];
    assert_eq!(v.pop(), Some("miranda"));
    assert_eq!(v.pop(), Some("carmen"));
    assert_eq!(v.pop(), None);

    // Get our command-line arguments as a vector of Strings.
    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
        println!("{}: {}", l,
            if l.len() % 2 == 0 {
                "functional"
            } else {
                "imperative"
            }
        );
    }


    // Slices
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];
    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    fn print(n: &[f64]) {
        for elt in n {
            println!("{}", elt);
        }
    }
    print(&v); // works on vectors
    print(&a); // works on arrays

    print(&v[0..2]); // print the first two elements of v
    print(&a[2..]); // print elements of a starting with a[2]
    print(&sv[1..3]); // print v[1] and v[2]
    print(&sa[..]);

    println!("In the room the women come and go,
    Singing of Mount Abora");

    println!(r#"In the room the women come and go,
  Singing of Mount Abora"#);

    println!("It was a bright, col\
    d day in April, and \
    there were four of us—\
    more or less.");

    let default_win_install_path = r"C:\Program Files\Gorillas";
    // let pattern = Regex::new(r"\d+(\.\d+)*");

    println!(r###"
This raw string started with 'r###"'.
Therefore it does not end until we reach a quote mark ('"')
followed immediately by three pound signs ('###'):
"###);
    println!(r###"
    This raw string started with 'r###"'.
    Therefore it does not end until we reach a quote mark ('"')
    followed immediately by three pound signs ('###'):
    "###);

    println!(r###"
        This raw string started with 'r###"'.
        Therefore it does not end until we reach a quote mark ('"')
        followed immediately by three pound signs ('###'):
        "###);


    // Byte Strings

    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);

    // assert_eq!("޵_޵".len(), 7); // 可能是复制之后字符发生了变化
    // assert_eq!("޵_޵".chars().count(), 3);

    // only make_ascii_uppercase and make_ascii_lowercase allow in &str
    let error_message = "too many pets".to_string();
    assert_eq!(format!("{}°{:02}′{:02}″N", 24, 5, 23), "24°05′23″N".to_string());

    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(", "), "veni, vidi, vici");

    assert!("ONE".to_lowercase() == "one");
    assert!("peanut".contains("nut"));
    assert_eq!("޵_޵".replace("޵", "■"), "■_■");
    assert_eq!(" clean\n".trim(), "clean");
    for word in "veni, vidi, vici".split(", ") {
        assert!(word.starts_with("v"));
    }

    
}
