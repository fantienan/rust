use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // 信道与所有权转移
    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     // send获取参数的所有权并移动这个值归接收者所有
    //     tx.send(val).unwrap();
    //     // println!("val is {}", val);
    // });
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    // 发送多个值并观察接收者的等待
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];
    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    // for received in rx {
    //     println!("Got: {}", received);
    // }

    // 通过克隆发送者来创建多个生产者
    let tx1 = tx.clone();
    thread::spawn(move || {
        let val = String::from("hi");
        tx1.send(val).unwrap();
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }

}
