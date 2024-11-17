/*
    变量
*/
fn main() {
    /**
     变量和可变性
     变量默认是不可变的（immutable）
     在变量前面加上mut(变换)，就是可变的
     */
    let mut x = 5;
    x = 6;
    println!("value is {0}", x);

    /**
     常量
     常量的命名约定是在单词之间使用全大写加下划线
     类似于不可变量了 let mut
     
     但是也有区别
     常量可以在任何作用域中声明，包括全局作用域
     常量只能被设置为常量表达式，而不可以是其他任何只能在运行时计算出的值
     */

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3小时就是 {0} 秒", THREE_HOURS_IN_SECONDS);


    /*
        隐藏
        定义一个与之前变量同名的新变量（第一个变量被第二个变量隐藏shadowing）

        隐藏与将变量标记为 mut 是有区别的
        隐藏相当于创建了一个新变量，可以改变值的类型
     */
    let y = 1;
    let y = y + 1;
    println!("隐藏了之前的变量y {}", y)
}