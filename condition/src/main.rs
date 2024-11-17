/*
    控制流
*/

fn main() {
    /*
        if 表达式

        条件必须是bool值，否则会报错
     */
    let number = 5;
    if number > 3 {
        println!("number的值比3大");
    } else if number == 3 {
        println!("number的值等于3")
    } else {
        println!("number的值比3小")
    };

    /*
        let语句中使用if

        代码块的值是其最后一个表达式的值
     */
    let condition = true;
    let x = if condition { 5 } else { 1 };


    /*
        循环
        loop while for
     */
    let mut index = 0;
    let _result = loop {
        index += 1;
        if index > 3 {
            break index;
        } 
    };
    println!("result is {}", _result);

    /*
        循环嵌套
        可以选择在一个循环上指定一个 循环标签（loop label）
     */
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            };
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    };

    /*
        while 条件循环

     */

    let mut count2 = 3;
    while count2 != 0 {
        println!("count2 is {count2}");
        count2 -= 1;
    };


    /*
        for 遍历集合
     */
    let arr = [1,2,3,4,5];
    for element in arr {
        println!("the value is: {element}");
    };

    // 1..4 不包含4
    for number2 in (1..4) {
        println!("{number2}");
    }
    for number2 in (1..4).rev() {
        println!("{number2}");
    }
}
