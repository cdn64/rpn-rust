const PLUS: i32 = 1;
const MINUS: i32 = 2;
const MULTIPLY: i32 = 3;

pub struct Token {
    operator: bool,
    value: i32,
}
impl Token {
    fn from(expression: &str) -> Token {
        let operator = match expression {
            "+" => PLUS,
            "-" => MINUS,
            "*" => MULTIPLY,
            _ => 0,
        };
        if operator != 0 {
            Token {
                operator: true,
                value: operator,
            }
        } else {
            Token {
                operator: false,
                value: expression.parse::<i32>().expect("Unparseable number"),
            }
        }
    }
    fn is_number(&self) -> bool {
        self.operator == false
    }
    fn is_operator(&self) -> bool {
        self.operator == true
    }
    fn result(&self, a: Token, b: Token) -> Token {
        if a.is_operator() || b.is_operator() {
            panic!("There are operators on the stack")
        }
        let result = match self.value {
            PLUS => a.value + b.value,
            MINUS => a.value - b.value,
            MULTIPLY => a.value * b.value,
            _ => panic!("Operator not supported"),
        };
        Token {
            operator: false,
            value: result,
        }
    }
}

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
            Ok(self.stack.last().unwrap().value)
        } else {
            Err(())
        }
    }
}

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
}
