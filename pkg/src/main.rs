/*
    使用包
    crate 是 Rust 在编译时最小的代码单位

    Cargo 遵循的一个约定：src/main.rs 就是一个与包同名的二进制 crate 的 crate 根
*/
use crate::garden::vegetables::Asparagus;

pub mod garden; // 告诉编译器应该包含在src/garden.rs文件中发现的代码

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
