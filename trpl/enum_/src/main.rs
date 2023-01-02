use std::option::Option;

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
    V4_(String),
    V6_(String)
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String
}

enum Message {
    Ouit,// 没有关联任何类型
    Move {x: i32, y:i32}, // 类似结构体包含命名字段
    Write(String),// 包含单个String
    ChangeColor(i32, i32,i32),// 包含三个i32
}
// 使用impl在枚举上定义方法
impl Message {
    fn call(&self) {
        
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?}{:?}", four, six);
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
    println!("{:?}{:?}", home, loopback);
    let home_ = IpAddrKind::V4_(String::from("127.0.0.1"));
    let loopback_ = IpAddrKind::V6_(String::from("::1"));
    println!("{:?}{:?}", home_, loopback_);
    let m = Message::Write(String::from("hello"));
    m.call();
    let x: Option<u32> = None;
    assert_eq!(x.is_none(), true);
    let x = Some(2);
    assert_eq!(x.is_some(), true);
    let y = Some(8);
    let sum = x + y; 

}
