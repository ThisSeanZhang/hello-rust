
// 以下两种引用方式是等价的
use std::{ cmp::Ordering, io};

// use std::io;
// use std::cmp::Ordering;

// 以下两种引用方式是等价的
// use std::io::{self, Write};
// use std::io;
// use std::io::Write;

// 可以使用*可以将某个目录的公共条目都导入作用域
use std::collections::*;

fn main() {

}