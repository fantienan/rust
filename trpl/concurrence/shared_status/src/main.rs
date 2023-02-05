use std::sync::{Mutex, Arc};
use std::thread;
// use std::rc::Rc;

fn main() {
    // 单线程使用互斥器
    // let m = Mutex::new(5);
    // {
    //     let mut num = m.lock().unwrap();
    //     * num = 6;
    // }
    // println!("m = {:#?}", m);

    // 在线程间共享 Mutex<T>
    // let counter = Mutex::new(0);
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 1..10 {
        // let counter = Rc::clone(&counter);// Rc不能安全的在线程间共享
        let counter = Arc::clone(&counter);// 使用Arc原子引用计数代替Rc，它可以安全的用于并发环境的类型
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", counter.lock().unwrap());
}
