/*
    定义模块

    在 Rust 中，默认所有项（函数、方法、结构体、枚举、模块和常量）对父模块都是私有的

    使用pub关键字暴露路径

    super 类似以 .. 语法开始一个文件系统路径
*/
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_paymoent() {}
    }
}

/*
    引用模块的路径

    绝对路径
    相对路径
*/

pub fn eat_at_resturant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

/*
    使用use关键字将路径引入作用域

    作用域中增加 use 和路径类似于在文件系统中创建软连接

    使用as提供别名
*/
use crate::front_of_house::hosting;
use crate::front_of_house::hosting as hosting1;

pub fn eat_at_resturant2() {
    hosting::add_to_waitlist();
    hosting1::add_to_waitlist();
}

/*
使用外部包

 */
use rand::Rng; // toml声明的
use std::collections::HashMap; // rust标准件
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}

use std::cmp::Ordering;
use std::io;
// 等同于 嵌套路径
use std::{cmp::Ordering, io};

use std::io;
use std::io::Write;
// 等同于
use std::io::{self, Write};

// glob运算符
use std::collections::*;