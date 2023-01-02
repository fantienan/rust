use rand::Rng;
use restaurant::front_of_house::hosting;
// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io};
// use std::io;
// use std::io::Write;
use std::io::{self, Write};
// 如果希望将一个路径下所有共有项引入作用域，可以指定路径后跟*，glob运算符
use std::collections::*;
fn main() {
    hosting::add_to_waitlist();
    let secret_number = rand::thread_rng().gen_range(1..=100);
}