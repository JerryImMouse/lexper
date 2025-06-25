#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum OperatorType {
    PLUS,
    MINUS,

    MULTIPLY,
    DIVIDE,

    MODULO,
    POWER,
}

impl OperatorType {
    pub fn precendance(&self) -> u8 {
        match self {
            Self::PLUS | Self::MINUS => 1,
            Self::MULTIPLY | Self::DIVIDE => 2,
            Self::POWER | Self::MODULO => 3,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum TokenType {
    LITERAL(f64),

    OPERATOR(OperatorType),

    LPAREN,
    RPAREN,

    IDENTIFIER(String), // e.g sin(), cos() etc.

    COMMA, // for future
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub r#type: TokenType,
    pub line: usize,
    pub col: usize,
}

impl Token {
    pub fn new(r#type: TokenType, line: usize, col: usize) -> Self {
        Self { r#type, line, col }
    }

    pub fn get_op(&self) -> Option<OperatorType> {
        match &self.r#type {
            TokenType::OPERATOR(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn get_type(&self) -> TokenType {
        self.r#type.clone()
    }

    pub fn ty(&self) -> &TokenType {
        &self.r#type
    }
}
