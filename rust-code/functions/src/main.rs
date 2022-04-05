fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeld_measurement(5, 'h');
    let num = five(4, 16);
    println!("{}", num);
    let u = {
        let y = 1;
        y
    };
    println!("The vaule of u is: {}", u)
}
fn another_function(x: i32) {
    println!("The value of x is: {}", x)
}
fn print_labeld_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {} {}", value, unit_label)
}
fn five(x: u32, y: u32) -> u32{
    (x + y)/2
}