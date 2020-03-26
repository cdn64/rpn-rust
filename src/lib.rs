// pub trait Token {
//     fn is_number(&self) -> bool;
//     fn is_operator(&self) -> bool;
// }

pub struct Token {
    operator: bool,
    value: i32,
}
impl Token {
    fn is_number(&self) -> bool {
        self.operator == false
    }
    fn is_operator(&self) -> bool {
        self.operator == true
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
    pub fn parse(&mut self, expression: &str) {
        println!("{}", expression);
        self.stack.push(Token {
            operator: false,
            value: 4,
        });
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
    fn it_works() {
        let mut stack = Stack::new();
        stack.parse("2 2 +");
        assert_eq!(stack.value().unwrap(), 4);
    }
}
