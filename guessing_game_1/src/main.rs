use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("数字游戏开始！ ");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("秘密数字是：{}", secret_number);

    loop {
        println!("请输入你的数字！");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取失败！");

        println!("你输入的数字是：{}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小了！"),
            Ordering::Greater => println!("大了！"),
            Ordering::Equal => {
                println!("猜对了！");
                break;
            }
        }
    }
}
