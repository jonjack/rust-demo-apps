use utils::{math, strings};

// This tells the compiler to look for either utils.rs or utils/mod.rs
mod utils;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let sum = utils::math::sum_numbers(&numbers);
    println!("Sum: {}", sum);

    let avg = math::average(&numbers);
    println!("Average: {}", avg);

    let greeting = strings::greet("Alice");
    println!("{}", greeting);
}
