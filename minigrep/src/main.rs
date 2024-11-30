/*
    文件路径和要搜索的字符串
*/
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // 读取参数值
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // cargo run -- test sample.txt
    println!("Searching for {}", config.query);
    println!("in file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
