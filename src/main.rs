mod eval;
mod lexer;
mod parser;

mod error;

pub use error::{Error, Result};

use crate::{eval::Context, lexer::Lexer, parser::Parser};

fn main() -> Result<()> {
    let mut args = std::env::args();
    if args.len() != 2 {
        eprintln!("This program accepts only 1 argument, the expression itself");
        return Ok(());
    }
    args.next(); // consume path

    let expr = args
        .next()
        .expect("Expected an expression as a first argument");

    let mut lexer = Lexer::new(expr.to_string());
    lexer.lex()?;
    let mut parser = Parser::new(lexer.tokens());
    let expr = parser.parse_expression(0)?;

    let mut ctx = Context::new();
    ctx.init();

    let result = ctx.eval(expr)?;
    println!("{}", result);
    Ok(())
}
