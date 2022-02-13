// 求平均值
fn avg(a: u32, b: u32) -> u32 {
    (a & b) + ((a ^ b) >> 1)
}
// 数组
fn arr() {
    let array: [u32; 5] = [1,2,3,4,5];
    println!("array[1]={}", array[1]);
    let buffer: [u32; 32 * 1024] = [0; 32 * 1024];
    println!("buffer[1024]={}", buffer[1024]);
    let mut buffer = buffer;
    buffer[1] = 13
}
// 切片类型
fn slice() {
    let arr: [i32; 5] = [1,2,3,4,5];
    let slice = &arr[0..3];
    println!("slice[0]={}", slice[0]);
    println!("slice_len={}",slice.len());
    println!("slice_is_empty={}", slice.is_empty())
}
// 结构体
// 元组结构
struct Pair(i32, f32);
// 标准的C结构
#[derive(Debug)] // 派生属性 可以通过{:?}打印结构体的所有属性
struct Person {
    name: String,
    age: u32,
}
// 单元结构（无字段，通常在泛型中使用）
struct Unit;
fn structure() {
    println!("structure----");
    let pair = Pair(10, 4.2);
    println!("pair.0={}", pair.0);
    let jack = Person {
        name: String::from("jack"),
        age: 6
    };
    println!("name={}, age={}", jack.name, jack.age);
    println!("{:?}", jack);
    let unit = Unit;
}
// 枚举
enum IPAddr {
    IPv4(u8, u8, u8, u8),
    IPv6(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8),
}

fn enumeration() {
    println!("枚举----------------------");
    let localhost: IPAddr = IPAddr::IPv4(127,0,0,1);
    match localhost {
        IPAddr::IPv4(a,b,c,d) => {
            println!("{} {} {} {}", a,b,c,d);
        }
        _ => {}
    }
}

fn main() {
    assert_eq!(avg(4294967295, 4294967295), 4294967295);
    assert_eq!(avg(0, 0), 0);
    assert_eq!(avg(10, 20), 15);
    assert_eq!(avg(4294967295, 1), 2147483648);
    println!("passed");
    let a: i32 = 10;
    let b: char = 'A';
    let tuple = (a,b);
    println!("tuple.0 = {:?}", tuple.0);
    println!("tuple.1 = {:?}", tuple.1);
    arr();
    slice();
    structure();
    enumeration();
    println!("println函数---------------");

    println!("{}", 42);

    // 可以使用额外的位置参数.
    println!("{0}{1}{0}", 4, 2);

    // 使用命名参数.
    println!("name={name} age={age}", name="jack", age=6);

    // 可以在 `:` 后面指定特殊的格式.
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 可以按指定宽度来右对齐文本.
    println!("{number:>width$}", number=1, width=6);

    // 在数字左边补 0.下面语句输出 "000001".
    println!("{number:>0width$}", number=1, width=6);

    // println! 会检查使用到的参数数量是否正确.
    println!("My name is {0}, {1}, {0}", "Bond", "Uhhr");
    // 编译将会报错, 请补上漏掉的参数："James"
}

