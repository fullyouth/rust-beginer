/*
    方法语法
    在结构体的上下文中被定义，它们第一个参数总是 self
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// implementation 
// impl中的所有内容都将与Rectangle有关
// &self 实际上是 self: &Self 的缩写
// 方法可以和属性名相同
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(self: &Self, r2: &Rectangle) -> bool {
        self.width > r2.width && self.height > r2.height
    }

    // 不以 self为第一参数的关联函数 不是方法 是函数
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "r1 面积是 {} ",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    }; 
    println!(
        "r1 可以 包住 r2吗 {} ",
        rect1.can_hold(&rect2)
    );

    let _sq = Rectangle::square(3);
}


