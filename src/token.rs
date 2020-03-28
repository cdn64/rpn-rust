pub enum Token {
    Plus,
    Minus,
    Multiply,
    Divide,
    Number(f64),
}
impl Token {
    pub fn tokenize(expression: &str) -> Vec<Token> {
        expression.split(" ").map(|str| Token::from(str)).collect()
    }
    pub fn from(expression: &str) -> Token {
        match expression {
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Multiply,
            "/" => Token::Divide,
            _ => Token::Number(expression.parse::<f64>().expect("Unparseable number")),
        }
    }
}
