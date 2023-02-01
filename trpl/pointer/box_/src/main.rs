// 除了数据被储存在堆上而不是栈上之外，box 没有性能损失。不过也没有很多额外的功能。它们多用于如下场景：
// - 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
// - 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
// - 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候
use std::ops::Deref;
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use crate::List::{Cons, Nil};
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:#?}", list);

    let x = 5;
    // let y = &x;
    // let y = Box::new(x);
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
