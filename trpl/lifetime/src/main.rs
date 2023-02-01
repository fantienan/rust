// https://kaisery.github.io/trpl-zh-cn/ch10-03-lifetime-syntax.html
use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    // 声明周期碧莲悬垂引用
    {
        let r;
        {
            let x = 5;
            r = &x;
        }
    }
    // 尝试用用离开作用域的值
    // println!("{}", r);
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(&string1.as_str(), &string2);
    println!("The longset string {}", result);

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longset string {}", result);
    }

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest1(string1.as_str(), string2.as_str());
    }
    println!("The longset string {}", result);

    let novel = String::from("Call me Ishamael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    let s: &'static str = "I have a static lifetime.";
}
// 函数签名中的生命周期注解 告诉Rust 参数和返回的引用的生命周期与泛型函数的声明周期一样久
// 函数返回的引用的生命周期与传入该函数的引用的生命周期的较小者一致
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest1<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
// fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
