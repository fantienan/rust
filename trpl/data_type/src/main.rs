/***
 * 数据类型
 * 1. 标量类型：具有单独的值的类型
 * 整型、浮点型、布尔型、字符型
 * 2. 复合类型，rust中有两种原生的复合类型, 数组类型和元组类型
 * 元组：可有由多个不同类型的值组成，一旦声明，大小和类型不能改变，不带任何值的元组角单元元组
 * 数组：每个元素的类型必须相同，且长度固定
 *
 */
use std::io;
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{} {} {} {}", tup.0, x, y, z);
    let a = [1, 2, 3, 4];
    loop {
        let mut index = String::new();
        println!("请输入数组索引");
        io::stdin().read_line(&mut index).expect("读取数据失败");
        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let element = a[index];
        println!("{}", element);
        break;
    }
}
