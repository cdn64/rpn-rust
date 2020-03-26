use super::token::*;

pub struct Stack {
    stack: Vec<Token>,
}
impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: Vec::<Token>::new(),
        }
    }
    pub fn push(&mut self, token: Token) {
        if token.is_number() {
            self.stack.push(token)
        } else {
            // Apply change
            if self.stack.len() < 2 {
                panic!("Operator needs more two or more tokens on the stack");
            } else {
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(token.result(a, b));
            }
        }
    }
    pub fn parse(&mut self, expression: &str) {
        let string_parts: Vec<&str> = expression.split(" ").collect();
        for string_part in &string_parts {
            self.push(Token::from(string_part));
        }
    }
    pub fn value(self) -> Result<i32, ()> {
        if self.stack.len() > 0 {
            Ok(self.stack.last().unwrap().value())
        } else {
            Err(())
        }
    }
}
