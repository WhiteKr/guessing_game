pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let guess = Guess::new(42);
        assert_eq!(guess.value(), 42);
    }

    #[test]
    #[should_panic = "Guess value must be greater than or equal to 1, got 0."]
    fn new_panics_on_less_than_one() {
        Guess::new(0);
    }

    #[test]
    #[should_panic = "Guess value must be less than or equal to 100, got 101."]
    fn new_panics_on_greater_than_hundred() {
        Guess::new(101);
    }
}
