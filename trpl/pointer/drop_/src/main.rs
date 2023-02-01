use std::mem::drop;

struct CustomSmartPoniter {
    data: String,
}
impl Drop for CustomSmartPoniter {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPoniter with data `{}`!", self.data);
    }
}
fn main() {
    let c = CustomSmartPoniter {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPoniter {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPoniter created.");
    // 强制提早清理值
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.")
}
