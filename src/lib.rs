mod stack;
mod token;

pub use stack::*;
pub use token::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_example() {
        let mut stack = Stack::new();
        stack.parse("1 2 +");
        assert_eq!(stack.value().unwrap(), 3);
    }

    #[test]
    fn complex_example() {
        let mut stack = Stack::new();
        stack.parse("1 2 3 4 5 + - * +");
        assert_eq!(stack.value().unwrap(), -11);
    }

    #[test]
    fn parsing_multiple_times() {
        let mut stack = Stack::new();
        stack.parse("1");
        stack.parse("2 *");
        assert_eq!(stack.value().unwrap(), 2);
    }
}
