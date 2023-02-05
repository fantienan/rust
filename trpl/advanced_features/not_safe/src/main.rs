use std::slice;
// 静态变量
static mut COUNTER: u32 = 0;
// 实现不安全的trait
unsafe trait Foo {}
unsafe impl Foo for i32 {}
fn main() {
    // 解引用裸指针
    let mut num = 5;
    // let r1 = &num as *const i32;
    // let r2 = &mut num as *mut i32;

    unsafe {
        // println!("r1 is {}", r1);
        // println!("r2 is {}", r2);
        dangerous();
        println!("{}", abs(-3))
    }
    // 创建不安全代码的安全抽象，仅仅因为函数包含不安全代码并不意味着整个函数都需要标记为不安全的。
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = split_at_mut(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    add_to_count(3);
}

// 不安全函数
unsafe fn dangerous() {}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let prt = values.as_mut_ptr(); // 返回裸指针
    assert!(mid <= len);
    // (&mut values[..mid], &mut values[mid..]) // 会报错，需要用到不安全代码的安全抽象
    unsafe {
        (
            slice::from_raw_parts_mut(prt, mid),
            slice::from_raw_parts_mut(prt.add(mid), len - mid),
        )
    }
}
// 使用extern函数调用外部代码
extern "C" {
    fn abs(input: i32) -> i32;
}
