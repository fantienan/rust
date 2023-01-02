// 引用与借用
fn main() {
    // 引用：&符表示引用，它允许你使用值但不获取其所有权
    let s1 = String::from("hello");
    // &s1:创建指向s1的引用，但是并不用于它的所有权
    let len = calculate_lenght(&s1);
    println!("{}{}", s1, len);
    // 创建一个引用的行为称为借用，尝试修改借用的变量回报错，借用默认不允许修改引用的值
    // 可变引用
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("{}", s2);
    // 如果存在一个对该变量的可变引用，就不能再创建对该变量的引用。
    // 这样可以防止同一时间对同一数据存在多个可变引用，这样的好处是Rust可以在编译时就避免数据竞争（data race）
    // 下面代码会在编译时报错
    let mut s3 = String::from("hello");
    // let r1 = &mut s3;
    // let r2 = &mut s3;
    // println!("{}{}", r1,r2);
    {
        let r3 = &mut s3;
    }
    let r4 = &mut s3;
    println!("{}{}", r4);
}
fn calculate_lenght(s: &String) -> usize {
    s.len()
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
