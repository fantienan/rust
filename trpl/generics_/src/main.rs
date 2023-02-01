struct Point<T> {
    x: T,
    y: T
}
struct Point1<T,U> {
    x: T,
    y: U
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
enum Option<T> {
    Some(T),
    None,
}
struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2,Y2>(self,other:Point2<X2,Y2>) -> Point2<X1,Y1> {
        Point2 {x: self.x, y: other.y}
    }
}

fn main() {
    let number_list = vec![34,50,25,100,65];
    let result = largest(&number_list);
    println!("{}", result);
    let   number_list = vec![102,34,6000,89,54,2,43,8];
    let result = largest(&number_list);
    println!("{}", result);
    let integer = Point {x: 5, y: 5};
    let float = Point {x: 1.0, y: 4.0};
    let integer_and_float = Point1 {x: 1, y: 4.0};
    let p1 = Point2 {x: 5, y: 10.4};
    let p2 = Point2 {x: "hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
