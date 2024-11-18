/*
    结构体demo

    计算长方形的面积
*/
// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//     println!("长方形的长宽为：{0} {1}", width1, height1);

//     let area = calc_area(width1, height1);
//     println!("面积为: {area}")
// }

// fn calc_area(width: u32, height: u32) -> u32 {
//     width * height
// }

// 使用元组重构
// fn main() {
//     let rect1 = (30, 50);

//     let area = calc_area(rect1);
//     println!("面积为： {area}");
// }

// fn calc_area(rect: (u32, u32)) -> u32 {
//     rect.0 * rect.1
// }

// 使用结构体重构
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rect {
        width: 30,
        height: 50
    };
    println!("rect1 is {rect1:#?}");
    dbg!(&rect1);
    let area = calc_area(&rect1);
    println!("面积为： {area}");
}

fn calc_area(rect: &Rect) -> u32 {
    rect.width * rect.height
}
 