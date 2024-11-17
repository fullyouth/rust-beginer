/*
    数据类型
*/
fn main() {
    /*
       有2种数据类型子集：标量scalar 和 复合 compound
       rust是静态类型语言，编译时就必须知道所有变量的类型。

       当我们使用parse将String 转换成number时，必须添加类型注解，否则编译失败
    */
    let guess: u32 = "42".parse().expect("不是一个number");
    println!("{}", guess);

    /*
       标量类型
       代表一个单独的值
       Rust有4中基本的标量类型：整型，浮点型，布尔型，字符型

       整型：没有小数的数字
       u：32 表示一个占据32bit的无符号整数
       i: 32 表示一个占据32bit的有符号整数

       有符号 和 无符号 代表数字能否为负值

       长度	有符号	无符号
       8-bit	i8	    u8
       16-bit	i16	    u16
       32-bit	i32	    u32
       64-bit	i64	    u64
       128-bit	i128	u128
       arch	isize	usize (架构位)

       整型字面值表示方法
       数字字面值	        例子
       Decimal (十进制)	98_222
       Hex (十六进制)	    0xff
       Octal (八进制)	    0o77
       Binary (二进制)	    0b1111_0000
       Byte (单字节字符)(仅限于u8)	b'A'
    */

    let _n: isize = 0xff;

    /*
       浮点型（采用 IEEE-754 标准表示）
       带小数点的数字

       2种类型：
       f32 单精度
       f64(默认) 双精度
       所有的浮点类型都是有符号的
    */

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    /*
       数值运算

    */
    let sum = 5 + 10;
    println!("sum is {}", sum);
    let diff = 95.5 - 4.3;
    println!("diff is {}", diff);
    let p = 4 * 30;
    println!("p is {}", p);
    let q = 56.7 / 32.2;
    println!("q is {}", q);
    let q2 = 5 / 3; // 整数除法会 向0舍入
    println!("q2 is {}", q2);
    let q3 = -5 / 3; // 整数除法会 向0舍入
    println!("q3 is {}", q3);
    let rem = 43 % 3;
    println!("rem is {}", rem);

    /*
    布尔类型
    true  false
     */
    let t = true;
    let f: bool = false; // 带有清晰的类型标注

    /*
       字符类型
       char类型是最原生的字母类型
       单引号声明 char 字面量
       双引号声明字符串字面量

       Rust 的 char 类型的大小为四个字节 (four bytes)，并代表了一个 Unicode 标量值（Unicode Scalar Value）
    */
    let c = 'z';
    let z: char = 'z';

    /*
       复合类型
       可以将多个值组合成一个类型

       Rust有2种原生的复合类型：元组tuple 和 数组array

       元组类型
       将多个其他类型的值组合金一个复合类型的主要方法
       长度固定：一旦声明，其长度不会增大或缩小
       使用（,,,）创建一个元组，每个位置的值类型可以不同

       不带任何值的元组有个特殊的名称，叫做 单元（unit） 元组 写作() 表示空
       如果表达式不返回任何其他值，则会隐式返回单元值
    */
    let tup: (i32, f64, u8) = (500, 1.4, 1);
    // 获取元组的值 解构（destructing）
    let (a, b, c) = tup;
    println!("a is {0}, b is {1}, c is {2}", a, b, c);
    // 也可以使用 x.1 来访问
    println!("b is {}", tup.1);


    /*
        数组类型
        与元组不同，数组中的每个元素的类型必须相同
        Rust的数组长度是固定的

        当你想要在栈（stack）而不是在堆（heap）上为数据分配空间,使用数组
        当确认数组长度时，数组更有用。

        编写数组的类型
        let m: [&str; 12] = 

        访问数组元素
     */
    let arr = [1,2,3,4,5];
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    // 设置初始值 和 5个相同的3
    let a = [3; 5];
}
