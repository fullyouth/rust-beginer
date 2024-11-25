/*
    枚举enums 和 模式匹配
*/

enum IpAddrKind{
    V4,
    V6
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    /*
        枚举值
        注意枚举的成员位于其标识符的命名空间中，并使用两个冒号分开
     */
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    fn route(ip_kind: IpAddrKind) {

    }
    route(IpAddrKind::V4)

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    }

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    }
}
