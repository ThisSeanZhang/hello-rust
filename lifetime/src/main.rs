fn main() {
    // println!("Hello, world!");
    // {
    //     let r;
    //     {
    //         let x = 5;
    //         r = &x; //借用x
    //     }// 但到此处时x生命到期了  编译错误
    //     println!("r: {}", r);
    // }

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // 使用具有不同生命周期的String来调用longest函数
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // 尝试在string2离开作用域后使用result
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // 为了使println语句中的result是有效的,string2需要保持到此有效 但因为提前释放所以编译不通过
    println!("The longest string is {}", result);
}
// 方法无法判断返回哪个引用， 也就无法得知用哪个引用来验证生命周期
// 不过我们也无法判断
/**
* 生命周期标注并不会改变任何引用的生命周期长度.
* 如同使用了泛型参数的哈桑农户可以接收任何类型一样,
* 使用了泛型生命周期的函数也可以接收带有任何生命周期的引用
* 在不影响生命周期的前提下标注本身
* 生命周期的标注使用了一种明显不同的语法: 它们的参数名称必须以撇号(')开头,通常使用全小写字符.名称简短.
* 'a被大部分开发者选择作为默认使用的名称
* 会将生命周期参数的标注填写在&引用运算符之后,并通过一个空格符来将标注引用与引用类型分开来
*/
// &i32  引用
// &'a i32 拥有显式生命周期的引用
// &'a mut i32 拥有显式生命周期的可变引用
/*
以下的生命周期标注表示了签名中所有的引用都必须拥有相同的生命周期
 */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}