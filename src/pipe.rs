use std::fmt::Display;

pub fn pipe<T: Display>(input: T) -> T {
    println!("input: {}", input);
    input
}
