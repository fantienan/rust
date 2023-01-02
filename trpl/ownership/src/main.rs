// 所有权 https://kaisery.github.io/trpl-zh-cn/ch04-01-what-is-ownership.html
// 1. Rust中每一个值都有一个所有者（owner）
// 2. 值在任一时刻有且只有一个所有者
// 3. 当所有者（变量）离开作用域，这个值将被丢弃
fn main() {
    // 字面值字符串 不可变
    let s = "Hello world";
    // String字符串 可变
    let mut s1 = String::from("Hello world");
    s1.push_str("world");
    println!("{}{}", s, s1);
    // 变量与数据的交互方式
    // 1. 移动
    let s2 = String::from("s2");
    let s3 = s2; // s2 移动
    let s4 = 1; // 值存在栈中 不会引动
    let s5 = s4;
    println!("{}{}{}{}", s3, s4, s5);
    // 2. 克隆 https://kaisery.github.io/trpl-zh-cn/ch04-01-what-is-ownership.html#%E5%8F%98%E9%87%8F%E4%B8%8E%E6%95%B0%E6%8D%AE%E4%BA%A4%E4%BA%92%E7%9A%84%E6%96%B9%E5%BC%8F%E4%BA%8C%E5%85%8B%E9%9A%86

    // 返回值和作用域
    let a = gives_ownership();
    let a1 = String::form("hello");
    let a2 = takes_and_gives_back(a1);
    // 使用一个值但不获取所有权
    let b = String::form("hello");
    let (b1, len) = calcuate_length(b);

}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
fn calcuate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
