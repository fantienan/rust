// Rust 中有三种循环方式：loop、while、for
fn main() {
    // 从循环中返回值
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result is {result}");
    // 循环标签：在多个循环质检消除歧义
    // 如果存在嵌套循环，break和continue应用于此时最内层的循环。可以选择在一个循环上指定一个循环标签，然后将标签与break和continue一起使用，
    // 使这些关键字应用于已标记的循环而不是最内层的循环
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count = {count}");
    while count != 10 {
        count += 1;
        println!("count = {count}")
    }
    let arr = [1,2,3,4,5];
    let mut index = 0;
    while index < 5 {
        println!("{}", arr[index]);
        index += 1;
    }
    for element in arr {
        println!("{}", element)
    }
    for element in (1..=4).rev() {
        println!("{}", element)
    }
}
