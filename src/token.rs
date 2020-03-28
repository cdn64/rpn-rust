pub enum Token {
    Plus,
    Minus,
    Multiply,
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
            _ => Token::Number(expression.parse::<f64>().expect("Unparseable number")),
        }
    }
}
