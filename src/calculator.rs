use super::token::*;

pub struct Calculator {
    stack: Vec<f64>,
}
impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            stack: Vec::<f64>::new(),
        }
    }
    pub fn value(&mut self) -> Option<f64> {
        self.stack.last().map(|i| *i)
    }
    pub fn calculate(&mut self, expression: &str) -> Option<f64> {
        for token in Token::tokenize(expression) {
            match token {
                Token::Number(n) => self.stack.push(n),
                Token::Addition => self.apply_binary_operator(&|a, b| a + b),
                Token::Subtraction => self.apply_binary_operator(&|a, b| a - b),
                Token::Multiplication => self.apply_binary_operator(&|a, b| a * b),
                Token::Division => self.apply_binary_operator(&|a, b| a / b),
                Token::SquareRoot => self.apply_unnary_operator(&|a| a.sqrt()),
            }
        }
        self.value()
    }
    pub fn apply_unnary_operator(&mut self, operation: &dyn Fn(f64) -> f64) {
        self.ensure_stack_size(1);
        let a = self.stack.pop().unwrap();
        self.stack.push(operation(a));
    }
    pub fn apply_binary_operator(&mut self, operation: &dyn Fn(f64, f64) -> f64) {
        self.ensure_stack_size(2);
        let (b, a) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
        self.stack.push(operation(a, b));
    }
    fn ensure_stack_size(&mut self, desired_size: usize) {
        let stack_size = self.stack.len();
        if stack_size < desired_size {
            panic!(
                "Current stack size of {} doesn't support this operation",
                stack_size
            )
        }
    }
}
