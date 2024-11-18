/*
    结构体
    自定义数据类型
*/

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // 创建User类型
    let mut user1 = User {
        active: true,
        username: String::from("haoqizhang"),
        email: String::from("xxx@163.com"),
        sign_in_count: 1,
    };
    // 获取结构体的值 使用点号
    user1.email = String::from("xxx@qq.com");

    let user2 = User {
        ..user1
    };

    fn build_user(emial: String, username: String) -> User {
        User {
            active: true,
            username: username,
            email: emial,
            sign_in_count: 1,
        }
    }

    // build_user("haoqi", "haoqi");



    /*
        元组结构体
    */
    struct Color(i32, i32, i32);
    let _black = Color(0,0,0);


    /*
        没有字段的结构体
     */
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}
