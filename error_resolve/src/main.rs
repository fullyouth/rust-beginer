/*
    Rust将错误分为2大类：可恢复的（recoverable）和不可恢复的（unrecoverable）

    Rust没有异常，而是有Result<T,E>类型，用于处理可恢复的错误

    使用panic!处理不可恢复的错误
*/

use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    // 直接调用panic!
    // panic!("crash and burn");

    // 使用panic!的backtrace
    let v = vec![1, 2, 3];
    // v[99];


    // 使用Result处理可恢复的错误
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    // unwrap 和 expect
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    // 传播错误
    read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new("");

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
