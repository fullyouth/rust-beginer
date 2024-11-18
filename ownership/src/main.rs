/*
    所有权
    有权（系统）是 Rust 最为与众不同的特性，对语言的其他部分有着深刻含义
    它让 Rust 无需垃圾回收（garbage collector）即可保障内存安全

    所有权（ownership）是 Rust 用于如何管理内存的一组规则

*/
fn main() {
    /*
       所有权规则

       1. Rust中每一个值都有一个所有者
       2. 值在任一时刻有且只有一个所有者
       3. 当所有者离开作用域，这个值将被丢弃
    */

    /*
       变量作用域
       作用域 是 一个 项item 在程序中有效的范围
    */
    {
        // 这里s无效，还没有声明
        let s = "hello";
        // 可以使用s
        println!("{}", s);
    } // 这里s无效，作用域结束
      // not found in this scope
      // println!("{}", s);

    /*
       String
       字符串字面值
       不可变性
       在编译时就知道长度，会被硬编码进最终的可执行文件中。
       这样的字符串字面值快速高效。

       为了支持一个可变，可增长的文本，需要在堆上分配一块在编译时未知大小的内存来存放内容。
       1.运行时向内存分配器请求内存
       2.需要一个当我们处理完String时将内存返回给分配器的方法

       步骤1可以有String::from时请求内存
       步骤2垃圾回收的问题，Rust采用【内存在变量离开作用域后就被自动释放】

       Rust 在结尾的 } 处自动调用 drop
    */
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    /*
       变量与数据的交互方式
       1.移动
       Rust 永远也不会自动创建数据的 “深拷贝”
       2.克隆
       3.拷贝

       https://kaisery.github.io/trpl-zh-cn/ch04-01-what-is-ownership.html
    */
    // 移动
    let s1 = String::from("hello");
    let s2 = s1;
    // 此时的s1就已经无效了 s1移动到了s2
    // println!("{s1}");
    println!("{s2}");

    // 克隆
    let s3 = s2.clone();
    println!("{s3}");

    // 拷贝
    // 只在栈上的数据
    let t1 = 5;
    let t2 = t1;
    // 这里的t1是有效的

    // 如下是一些 Copy 的类型：
    // 所有整数类型，比如 u32。
    // 布尔类型，bool，它的值是 true 和 false。
    // 所有浮点数类型，比如 f64。
    // 字符类型，char。
    // 元组，当且仅当其包含的类型也都实现 Copy 的时候。比如，(i32, i32) 实现了 Copy，但 (i32, String) 就没有。

    /*
       所有权与函数
    */
    let z = String::from("hello z");
    takes_owership(z);
    // TODO 这就不能在访问z了 这个不是太理解
    // println!("{z}");

    fn takes_owership(str: String) {
        println!("{str}");
    }
    fn makes_copy(num: i32) {
        println!("{num}");
    }

    let z2 = 2;
    makes_copy(z2);

    /*
       返回值与作用域
       返回值也可以转移所有权
    */
    let r1 = gives_ownership(); // gives_ownership 将返回值
                                // 转移给 s1
    let r2 = String::from("hello"); // s2 进入作用域
    let r3 = takes_and_gives_back(r2);

    fn gives_ownership() -> String {
        // gives_ownership 会将
        // 返回值移动给
        // 调用它的函数
        let some_string = String::from("yours"); // some_string 进入作用域。
        some_string // 返回 some_string
                    // 并移出给调用的函数
    }

    // takes_and_gives_back 将传入字符串并返回该值
    fn takes_and_gives_back(a_string: String) -> String {
        // a_string 进入作用域
        a_string // 返回 a_string 并移出给调用的函数
    }

    /*
       引用reference
        & 符号就是 引用

        1在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
        2引用必须总是有效的。
    */
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    /*
       借用borrowing
       尝试修改借用的值，错误
    */
    let s = String::from("hello");
    change(&s); // 这个&s就是借用

    fn change(s: &String) {
        s.push_str(", world")
    }

    let s = String::from("hello");
    change(&mut s); // 这里就是可变引用

    fn change(s: &mut String) {
        s.push_str(", world")
    }

    // 可变引用只能有一个 避免数据竞争
    let r1 = &mut s;
    let r2 = &mut s;

    // 可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能 同时 拥有
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;

    // 也 不能在拥有不可变引用的同时拥有可变引用
    // 不可变引用的用户可不希望在他们的眼皮底下值就被意外的改变了

    // 悬垂引用
    // 其指向的内存可能已经被分配给其它持有者
    fn dangle() -> &String {
        // dangle 返回一个字符串的引用

        let s = String::from("hello"); // s 是一个新字符串

        // 返回字符串 s 的引用
    } // 这里 s 离开作用域并被丢弃。其内存被释放。
      // 危险！


      /*
        Slice类型
        slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。slice 是一种引用，所以它没有所有权。
       */
}
