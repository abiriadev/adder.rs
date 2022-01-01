#[allow(dead_code)]
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Invalid guess value! you must provide more than 1");
        } else if value > 100 {
            panic!("Invalid guess value! you must provide less than 100");
        }

        Guess { value }
    }
}
