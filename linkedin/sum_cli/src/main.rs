/**
 * cargo run -- 1 2 3 4
 * cargo run -- 10 20 30 40 -40 -30 -20 -10
 * cargo run -- 10 20 30 "fred" (panic!)
 */
use std::env;

fn main() {
    let sum: i32 = env::args().skip(1).fold(0, |acc, x| {
        acc + x.parse::<i32>().expect("All args must be integers.")
    });
    println!("The sum is {}", sum);
}
