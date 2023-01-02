// Slice 类型
fn main() {
    let mut s =String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    println!("{}{}", hello, world);
    let word = first_word(&s);
    // s.clear();
    println!("{}", word)
}
// 编写一个函数，该函数接收一个用空格分隔单词的字符串，并返回在该字符串中找到的第一个单词。如果函数在该字符串中并未找到空格，则整个字符串就是一个单词，所以应该返回整个字符串。o
// &str 是不可变引用
fn first_word(s: &str) -> &str {
    // 将String转换为字节数组
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}