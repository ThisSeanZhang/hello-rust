// 一般使用lib进行定义接口   main函数进行简单调用运行
/*
代码中的internal_adder函数没有被标注为pub，但
因为测试本身就是Rust代码，并且tests模块就是Rust模块，所以你
可以正常地将internal_adder导入测试作用域并调用它
 */
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}