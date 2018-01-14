extern crate rustd;


fn main() {
    println!("Fibonacci for 35 is {}", fib(35));
}

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2)
    }
}
