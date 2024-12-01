fn main() {
    // 数据类型
    // 有符号整数为： i8 、 i16 、 i32 、 i64 、 i128和isize 。
    // 无符号整数为： u8 、 u16 、 u32 、 u64 、 u128和usize 。
    // u64: 0 ~ 2^64 - 1
    // i64: -2^63 至 2^63 - 1 
    // char 使用 单引号 '' 所有的chars都使用4个字节的内存
    let my_number = 1000000;
    println!("{}", my_number as isize);
    // 基础字符和符号使用4个字节中的1个
    // 部分使用4个中的2个
    // 中日韩文使用4个中的3-4个
    println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes
    println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
    println!("Size of string containing 'ß': {}", "ß".len());
    println!("Size of string containing '国': {}", "国".len());
    println!("Size of string containing '𓅱': {}", "𓅱".len());
}
