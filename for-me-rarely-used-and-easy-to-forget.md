# Environment Variables
```rust
use std::env;

let key = "HOME";
match env::var_os(key) {
    Some(val) => println!("{}: {:?}", key, val),
    None => println!("{} is not defined in the environment.", key)
}
```

# Show Error Message
```rust
enum MyErr {
    Reason1(String),
    Reason2(String, u32),
}
impl fmt::Display for MyErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyErr::Reason1(ref s) => 
                write!(f, "`{}` is the error", s),
            MyErr::Reason2(ref s, ref num) => 
                write!(f, "`{}` and `{}` are error", s, num),
        }
    }
}
```

# Standard error
`println!()` is the standard output, and `eprintln!()` is the standard error.