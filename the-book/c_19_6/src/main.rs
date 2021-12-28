use proc_macro::TokenStream;

fn main() {
    /// 宏

    /// 用于通用元编程的macro_rules! 声明宏

    #[macro_export]
    macro_rules! vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                 $(temp_vec.push($x);)*
                 temp_vec
            }
        };
    }
    let vec1 = vec![1, 2, 3];

    // #[some_attribute]
    // pub fn some_name(input: TokenStream) -> TokenStream {
    //
    // }
}
