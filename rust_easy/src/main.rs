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

    // 类型推断
    // 编译器可以自动推断
    let _my_number: u8 = 8;
    // 对于数字 可以把类型放在数字后面 也可以使用下划线
    let my_number2 = 8u8;
    let my_number3 = 8_u8;
    let my_number4 = 8_________u8;
    println!("{my_number2}");
    println!("{my_number3}");
    println!("{my_number4}");
    // 最大值 最小值
    println!("The smallest i8 is {} and the biggest i8 is {}.", i8::MIN, i8::MAX); // hint: printing std::i8::MIN means "print MIN inside of the i8 section in the standard library"
    println!("The smallest u8 is {} and the biggest u8 is {}.", u8::MIN, u8::MAX);
    println!("The smallest i16 is {} and the biggest i16 is {}.", i16::MIN, i16::MAX);
    println!("The smallest u16 is {} and the biggest u16 is {}.", u16::MIN, u16::MAX);
    println!("The smallest i32 is {} and the biggest i32 is {}.", i32::MIN, i32::MAX);
    println!("The smallest u32 is {} and the biggest u32 is {}.", u32::MIN, u32::MAX);
    println!("The smallest i64 is {} and the biggest i64 is {}.", i64::MIN, i64::MAX);
    println!("The smallest u64 is {} and the biggest u64 is {}.", u64::MIN, u64::MAX);
    println!("The smallest i128 is {} and the biggest i128 is {}.", i128::MIN, i128::MAX);
    println!("The smallest u128 is {} and the biggest u128 is {}.", u128::MIN, u128::MAX);

    // 浮点数
    // 5.5 5.0 5. 是浮点数
    let my_float: f64 = 5.0;
    // 浮点数类型 f32 f64;  
    let my_other_float: f32 = 5.5;
    // f32 f64类型不同  不能相加
    let _r = my_float + my_other_float;

     // println是一个宏  后面使用！表示是一个宏
    // {}表示把变量放在这里
    println!("Hello, worlds number {} and {}!", 8, 9);
    // -> 表示函数返回值类型
    fn number() -> i32 {
        8
    }
    println!("Hello, world number {}!", number());
    // 变量名:类型
    fn multiply(number_one: i32, number_two: i32) -> i32 {
        // Two i32s will enter the function. We will call them number_one and number_two.
        let result = number_one * number_two;
        println!("{} times {} is {}", number_one, number_two, result);
        result
    }
    let multiply_result = multiply(8, 9);
    println!("multiply_result {}!", multiply(5, 6));

     // 定义变量和代码块
     {
        // 变量的使用范围是在代码块中结束的
        let my_number = 8;
        println!("Hello, number {}", my_number);
    }
    // 这里找不到my_number
    // println!("Hello, number {:?}", my_number);
    
    // 打印调试 某些类型是不能打印的 如() 添加:? 即可打印
    let doesnt_print = ();
    println!("This will not print: {:?}", doesnt_print); // ⚠️
    
    // 可变性
    let my_number = 8;
    my_number = 10; // error
    let mut my_number = 8;
    my_number = 10; // √
    // 但是不能改变类型
    let mut my_number = 8;
    my_number = "hello"; // error

    // 隐藏
   let x = 1; // 不是销毁了，而是被隐藏了
   let x = "hello";
   println!("{x}");
   
   let x2 = 1; // 不在同一个代码块，没有被隐藏
   {
    let x2 = "hello";
   }
   println!("{x2}");
}
