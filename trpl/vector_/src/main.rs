enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}
fn main() {
    // vector 只存储相同类型多于一个的值；
    let v: Vec<i32> = Vec::new();
    // vec!宏，根据我们提供的值来创建一个新的vector
    let v1 = vec![1, 2, 3];
    // 更新vector
    let mut v2 = Vec::new();
    v2.push(4);
    {
        let v3 = vec![1, 2, 3];
    }
    let v4 = vec![1, 2, 3, 4, 5];
    let third1 = &v4[2];
    println!("{:#?}", v4);
    match v4.get(2) {
        Some(third) => println!("{}", third),
        None => println!("..."),
    }
    let v5 = vec![1,2,3,4,5];
    // let does_not_exist = &v5[100];
    let does_not_exist = v.get(100);

    let mut v6 = vec![1,2,3,4,5];
    let first = &v6[0];
    // v.push(6);
    println!("{}", first);
    let v = vec![100,32,57];
    for i in &v {
        println!("{}", i)
    }
    let mut v = vec![100,32,57];
    for i in &mut v{
        *i += 50
    }
    // 使用枚举以便在vector中储存多种类型的值
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
