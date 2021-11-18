use test_adder_2;
// cargo test --test integration_test
// 仅想测试一个test文件时 传入的文件名
mod common2;
#[test]
fn it_adds_two() {
    common2::setup();
    assert_eq!(4, test_adder_2::add_two(2));
}