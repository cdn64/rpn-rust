pub enum Token {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    SquareRoot,
    Number(f64),
}
impl Token {
    pub fn tokenize(expression: &str) -> Vec<Token> {
        expression.split(" ").map(|str| Token::from(str)).collect()
    }
    pub fn from(expression: &str) -> Token {
        match expression {
            "+" => Token::Addition,
            "-" => Token::Subtraction,
            "*" => Token::Multiplication,
            "/" => Token::Division,
            "sqrt" => Token::SquareRoot,
            _ => Token::Number(expression.parse::<f64>().expect("Unparseable number")),
        }
    }
}
