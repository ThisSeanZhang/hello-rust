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
}
