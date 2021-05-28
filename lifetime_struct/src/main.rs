struct ImportantExcerpt<'a> {
     part: &'a str,
}
fn main() {
     let novel = String::from("Call me Ishmael. Some years ago...");
     let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
     println!("first_sentence: {}", first_sentence);
     let i = ImportantExcerpt { part: first_sentence };
}
/*
这个函数之所以使用了引用但没有进行生命周期标志却能通过
是因为历史原因，因为有些函数的生命周期是可预测的，所以简单的场景下可以自行推导
 */
fn first_word(s: &str) -> &str {
     let bytes = s.as_bytes();
     for (i, &item) in bytes.iter().enumerate() {
          if item == b' ' {
               return &s[0..i];
          }
     }
     &s[..]
}

/*
生命周期的推导遵循以下3条规则
第一条规则是，每一个引用参数都会拥有自己的生命周期参数。
换句话说，单参数函数拥有一个生命周期参数：fn foo<'a>(x: &'a
i32)；双参数函数拥有两个不同的生命周期参数：fn foo<'a, 'b>(x:
&'a i32, y: &'b i32)；以此类推。
第二条规则是，当只存在一个输入生命周期参数时，这个生命周
期会被赋予给所有输出生命周期参数，例如fn foo<'a>(x: &'a i32)
-> &'a i32。
第三条规则是，当拥有多个输入生命周期参数，而其中一个是
&self或&mut self时，self的生命周期会被赋予给所有的输出生命周
期参数。这条规则使方法更加易于阅读和编写，因为它省略了一些不
必要的符号。
 */