/*
    常见集合
    vector
    string
    hash map
*/

fn main() {
    learn_vector();
    learn_string();
    learn_hash_map();
}

fn learn_vector() {
    // 创建一个空的Vector类存储i32的类型的值
    // Vec<T> 是一个由标准库提供的类型，它可以存放任何类型
    let v: Vec<i32> = Vec::new();
    // vector 是用泛型实现的

    // vec! 宏
    let v2 = vec![1, 2, 3];

    // 如何修改一个 vector
    let mut v3: Vec<i32> = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    // 读取Vertor的元素
    let v4 = vec![1, 2, 3, 4, 5];
    let t3: &i32 = &v4[2];
    let t4: Option<&i32> = v4.get(3);
    println!("v4的第三个元素是 {t3}");
    match t4 {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    };

    // 遍历Vector
    let v5 = vec![100, 32, 56];
    for i in &v5 {
        println!("{i}");
    }

    // 使用枚举来储存多种类型
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn learn_string() {
    // 使用字符串来储存UTF-8编码的文本

    // 新建字符串
    let mut s1 = String::new();

    let data = "initial contents";

    let s2 = data.to_string();

    // 该方法也可直接用于字符串字面值：
    let s3 = "initial contents".to_string();
    // 也可以使用 String::from 函数来从字符串字面值创建 String
    let s4 = String::from("initial contents");

    let mut s5 = String::from("foo");
    s5.push_str("bar");
    println!("{s5}");

    let s6 = String::from("Hello, ");
    let s7 = String::from("world!");
    let s8 = s6 + &s7; // 注意 s6 被移动了，不能继续使用

    // 多个字符串连接
    let s10 = String::from("tic");
    let s11 = String::from("tac");
    let s12 = String::from("toe");
    let s13 = format!("{s10}-{s11}-{s12}");

    // 索引字符串
    // 错误和提示说明了全部问题：Rust 的字符串不支持索引
    let s14 = String::from("hello");
    // let h = s14[0];
    let s15 = &s14[0..4];

    // 遍历字符串
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}

fn learn_hash_map() {
    // 创建一个hash map
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 访问值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    // 覆盖值
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    
    // 没有值时再覆盖
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
}