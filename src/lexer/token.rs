/// Type introduced to avoid pasting a lot of types into [`TokenType`], operators moved to a
/// sub-enum, here it is.
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
    pub(crate) fn precendance(&self) -> u8 {
        match self {
            Self::PLUS | Self::MINUS => 1,
            Self::MULTIPLY | Self::DIVIDE => 2,
            Self::POWER | Self::MODULO => 3,
        }
    }
}

/// The token type, literal, operator, parenthesis etc.
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

/// Represents some valuable information for our [Parser][`crate::Parser`], operators, literals
/// etc.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub(crate) r#type: TokenType,
    pub(crate) line: usize,
    pub(crate) col: usize,
}

impl Token {
    /// Creates new token instance
    pub fn new(r#type: TokenType, line: usize, col: usize) -> Self {
        Self { r#type, line, col }
    }

    /// Checks if this token is operator and returns the operator type
    pub fn get_op(&self) -> Option<OperatorType> {
        match &self.r#type {
            TokenType::OPERATOR(x) => Some(x.clone()),
            _ => None,
        }
    }

    /// Clones this token's type and returns, avoid using this due to overhead, try [`ty()`][`Token::ty`]
    pub fn get_type(&self) -> TokenType {
        self.r#type.clone()
    }

    /// Returns a reference to a current token's type
    pub fn ty(&self) -> &TokenType {
        &self.r#type
    }
}
