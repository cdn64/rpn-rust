mod calculator;
mod token;

pub use calculator::*;
pub use token::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_example() {
        let mut calculator = Calculator::new();
        assert_eq!(calculator.calculate("1 2 +").unwrap(), 3.0);
    }

    #[test]
    fn complex_example() {
        let mut calculator = Calculator::new();
        assert_eq!(calculator.calculate("1 2 3 4 5 + - * +").unwrap(), -11.0);
    }

    #[test]
    fn division() {
        let mut calculator = Calculator::new();
        assert_eq!(calculator.calculate("3 6 /").unwrap(), 0.5);
    }

    #[test]
    fn square_root() {
        let mut calculator = Calculator::new();
        assert_eq!(calculator.calculate("3 6 + sqrt").unwrap(), 3.0);
    }

    #[test]
    fn parsing_multiple_times() {
        let mut calculator = Calculator::new();
        assert_eq!(calculator.calculate("1").unwrap(), 1.0);
        assert_eq!(calculator.calculate("2 *").unwrap(), 2.0);
    }
}
