// 自动派生
#[derive(Debug, PartialEq, Default)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // let point1 = Point {x: 10, y:10};
    // let point2 = Point {x: 10, y:10};
    // println!("{:?}", point1 == point2);
    let p = Point::default();
    println!("{:?}", p)
}
