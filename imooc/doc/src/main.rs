//! A main project provides fibonacci function

/// In mathematics, the Fibonacci numbers, commonly denoted Fn form a sequence, called the Fibonacci sequence, such that
/// each number is the sum of the two preceding ones, starting from 0 and 1. That is
/// ```
/// F(0) = 0
/// F(1) = 1
/// F(n) = F(n − 1) + F(n − 2)
/// ```
fn fibo(n: u32) -> u32 {
    if n== 0 || n == 1 {
        n
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}

fn main() {
    // Calculate fibo(10)
    println!("fibo(10) = {}", fibo(10));
    /*
    The result should be 55
    */
}
