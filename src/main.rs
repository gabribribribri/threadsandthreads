use std::time::Instant;

use engine::MultithreadedMap;

mod engine;
mod tools;

fn main() {
    let max = 10_000_000;

    let tomate: Vec<usize> = (2..max).collect();
    let start = Instant::now();
    tomate.map(8, tools::is_prime_trial);
    println!("{:?}", start.elapsed());

    let tomate: Vec<usize> = (2..max).collect();
    let start = Instant::now();
    tools::is_prime_erat(tomate);
    println!("{:?}", start.elapsed());
}
