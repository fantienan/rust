fn main() {
    // 常用引发不可恢复错误的方法
    // panic! 宏
    // panic!("error!");
    // 断言
    // assert!(1 == 2);
    // assert_eq!(1,2);
    // 未实现的代码  
    // unimplemented!() 
    // 不应当被访问到的代码
    // unreachable!()

    // 可恢复错误
    let r = std::fs::read("/tmp/tool");
    match r {
        Ok(data) => println!("{:?}", data),
        Err(err) => println!("{:?}", err)
    }
}
