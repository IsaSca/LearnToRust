use crate::hello::hello_world;

mod hello;

fn main() {
    for i in 0..9 {
        println!("{} squared is {}", i, factorial(i as u64));
    }
    hello_world()
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n-1)
    }
}
