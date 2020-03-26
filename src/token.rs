const PLUS: i32 = 1;
const MINUS: i32 = 2;
const MULTIPLY: i32 = 3;

pub struct Token {
    operator: bool,
    value: i32,
}
impl Token {
    pub fn value(&self) -> i32 {
        self.value
    }
    pub fn from(expression: &str) -> Token {
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
    pub fn is_number(&self) -> bool {
        self.operator == false
    }
    pub fn is_operator(&self) -> bool {
        self.operator == true
    }
    pub fn result(&self, a: Token, b: Token) -> Token {
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
