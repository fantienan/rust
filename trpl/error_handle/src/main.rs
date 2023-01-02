use std::fs::{self, File};
use std::error::Error;
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    // 使用panic!的backtrace
    let v = vec![1,2,3];
    // v[99];
    // Result 处理可恢复的错误
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem opening the file {:?}", e)
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error)
            }
        }
    };
    // 失败时panic简写：unwrap和expect
    // 如果Result值是成员Ok，unwrap返回Ok中的值，如果Result是成员Err，unwrap会为我们调用panic!
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("打开hello.txt失败");
    // 传播错误
    let f = read_username_from_file();
    println!("{:?}", f);
    // 传播错误的简写：?运算符
    let f = read_username_from_file_();
    println!("{:#?}", f);
    let f = fs::read_to_string("helloi.txt");
    println!("{:#?}", f);
    let f = File::open("hello1.txt")?;
    Ok(())
}
// 从给定文本中返回第一行最后一个字符的函数的例子
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
fn read_username_from_file_() -> Result<String, io::Error> {
    let mut f = File::open("hello1.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
// 将错误返回给调用者
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}