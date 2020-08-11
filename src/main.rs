mod array_stuff;
mod hello;

use crate::array_stuff::array_test;
use crate::hello::hello_world;
use std::f64::consts;
use std::f64::consts::PI;



fn main() {
    println!("{}",use_pi());
    for i in 0..10 {
        println!("{}", factorial(i));
    }
    hello_world();
    array_test();
}

fn use_pi() -> f64{
    PI
}

fn factorial(x: u64) -> u64 {
    if x == 0 {
        1
    } else {
        x * factorial(x-1)
    }

}
