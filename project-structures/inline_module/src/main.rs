mod math {
    pub fn sum_numbers(numbers: &[i32]) -> i32 {
        numbers.iter().sum()
    }

    pub fn average(numbers: &[i32]) -> f64 {
        let sum = sum_numbers(numbers);
        sum as f64 / numbers.len() as f64
    }
}

mod strings {
    pub fn greet(name: &str) -> String {
        format!("Hello, {}!", name)
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let sum = math::sum_numbers(&numbers);
    println!("Sum: {}", sum);

    let avg = math::average(&numbers);
    println!("Average: {}", avg);

    let greeting = strings::greet("Alice");
    println!("{}", greeting);
}
