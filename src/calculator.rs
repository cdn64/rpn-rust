use super::token::*;

pub struct Calculator {
    stack: Vec<i32>,
}
impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            stack: Vec::<i32>::new(),
        }
    }
    pub fn value(&mut self) -> Option<i32> {
        self.stack.last().map(|i| *i)
    }
    pub fn calculate(&mut self, expression: &str) -> Option<i32> {
        for token in Token::tokenize(expression) {
            match token {
                Token::Number(n) => self.stack.push(n),
                Token::Plus => self.apply_binary_operator(&|a, b| a + b),
                Token::Minus => self.apply_binary_operator(&|a, b| a - b),
                Token::Multiply => self.apply_binary_operator(&|a, b| a * b),
            }
        }
        self.value()
    }
    pub fn apply_binary_operator(&mut self, operation: &dyn Fn(i32, i32) -> i32) {
        let stack_size = self.stack.len();
        if stack_size < 2 {
            panic!(
                "Current stack size of {} doesn't support binary operation",
                stack_size
            )
        }
        let (b, a) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
        self.stack.push(operation(a, b));
    }
}
