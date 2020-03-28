pub enum Token {
    Plus,
    Minus,
    Multiply,
    Number(i32),
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
            _ => Token::Number(expression.parse::<i32>().expect("Unparseable number")),
        }
    }
}
