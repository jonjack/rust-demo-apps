pub mod math {
    pub fn sum_numbers(numbers: &[i32]) -> i32 {
        numbers.iter().sum()
    }

    pub fn average(numbers: &[i32]) -> f64 {
        let sum = sum_numbers(numbers);
        sum as f64 / numbers.len() as f64
    }
}

pub mod strings {
    pub fn greet(name: &str) -> String {
        format!("Hello, {}!", name)
    }
}
