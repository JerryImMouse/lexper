//! Simple expression evaluation crate with custom lexer and parser. Currently doesn't have support
//! for a factorial, but this feature is in TODO list.
//!
//! ```
//! # use lexper;
//! let result = lexper::eval("sin(2) + 20^2").unwrap();
//! assert_eq!(f64::round(result), 401.0);
//! ```

mod eval;
pub(crate) mod lexer;
mod r#macro;
pub(crate) mod parser;

pub mod error;
pub(crate) use error::Result;

pub use error::Error;
pub use eval::Context;
pub use lexer::{Lexer, OperatorType, Token, TokenType};
pub use parser::{Expression, Parser};

pub fn eval(expr: &str) -> Result<f64> {
    let mut lexer = Lexer::new(expr.to_string());
    lexer.lex()?;

    let mut parser = Parser::new(lexer.tokens());
    let result = parser.parse_expression(0)?;
    let mut ctx = Context::new();
    ctx.init();
    ctx.eval(result)
}
