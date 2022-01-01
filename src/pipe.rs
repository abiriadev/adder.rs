use std::fmt::Display;

#[allow(dead_code)]
pub fn pipe<T: Display>(input: T) -> T {
    println!("input: {}", input);
    input
}
