struct Point<T> {
    x: T,
    y: T,
}

impl<T: std::fmt::Display> std::fmt::Display for Point<T>  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn show<T: std::fmt::Display>(a: T) {
    println!("show: {}", a)
}

fn display(a: impl std::fmt::Display) {
    println!("show: {}", a)
}

fn main() {
    let point = Point {x: 10, y: 10};
    show(&point);
    display(point);
}
