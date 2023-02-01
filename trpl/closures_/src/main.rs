use std::thread;
use closures_::{Inventory, ShirtColor, Rectangle};

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);

    let expensive_closure = |x| x;
    let s = expensive_closure(String::from("hello"));
    // let n = expensive_closure(5);
    let mut list = Vec::new();
    list.push(4);
    // 闭包捕获不可变引用
    let only_borrows = || println!("From closure: {:?}", list);
    let mut borrows_mutably = || list.push(3);

    // 使用闭包移动数据所有权给新线程
    thread::spawn(move || println!("From thread: {:?}", list)).join().unwrap();
    let mut list = [
        Rectangle {width:10, height:1},
        Rectangle {width: 3, height: 5},
        Rectangle {width: 7, height: 12}
    ];
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
    // let mut sort_operations = vec![];
    // let value = String::from("by key called");
    // list.sort_by_key(|r| {
    //     sort_operations.push(value)
    //     r.width
    // })
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    })

}
