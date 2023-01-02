#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}
fn main() {
   let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{}", area(&rect1));
    println!("{:#?}", rect1);
    if rect1.width() {
        println!("{}", rect1.width);
    }
    println!("{}", rect1.area());
    let sq = Rectangle::square(3);
    println!("{:#?}", sq)

}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
