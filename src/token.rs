pub enum Token {
    Plus,
    Minus,
    Multiply,
    Number(i32),
}
impl Token {
    pub fn from(expression: &str) -> Token {
        match expression {
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Multiply,
            _ => Token::Number(expression.parse::<i32>().expect("Unparseable number")),
        }
    }
    pub fn result(&self, a: Token, b: Token) -> Token {
        if let (Token::Number(number_a), Token::Number(number_b)) = (a, b) {
            match self {
                Token::Plus => Token::Number(number_a + number_b),
                Token::Minus => Token::Number(number_a - number_b),
                Token::Multiply => Token::Number(number_a * number_b),
                _ => panic!("Operator not supported"),
            }
        } else {
            panic!("Operators on stack!")
        }
    }
}
