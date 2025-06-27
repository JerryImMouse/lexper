use crate::Result;
use crate::parser::Expression;
use std::{collections::HashMap, f64};

/// Expression context, links variables and their values(currently only floats).
#[derive(Debug, Default)]
pub struct Context {
    variables: HashMap<String, f64>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    /// Initializes [`Context`] with some builtin variables, you should call this one before using
    /// it. Of course if you don't need some constants from f64... well, don't call it?
    pub fn init(&mut self) {
        self.variables.insert("PI".to_string(), f64::consts::PI);
    }

    /// Defines a new variable(basically just inserts a new record into a hashmap).
    pub fn define(&mut self, name: String, value: f64) {
        self.variables.insert(name, value);
    }

    pub(crate) fn get_var(&self, name: &str) -> Option<f64> {
        self.variables.get(name).copied()
    }

    /// Evaluates the passed expression, usually you'll get one from the [Parser][`crate::Parser`]
    pub fn eval(&self, expr: Expression) -> Result<f64> {
        expr.eval(self)
    }
}

#[cfg(test)]
mod test {
    use crate::{lexer::Lexer, parser::Parser};

    use super::*;

    #[test]
    fn test_eval_simple() {
        let raw = "2 + 10";
        println!("Raw String: {}", raw);
        let mut lexer = Lexer::new(raw.to_string());
        lexer.lex().unwrap();

        println!("{:#?}", lexer.tokens());

        let mut parser = Parser::new(lexer.tokens());
        let expr = parser.parse_expression(0).unwrap();

        println!("\n{:#?}", expr);

        let mut ctx = Context::new();
        ctx.init();

        let result = ctx.eval(expr).unwrap();
        println!("Result: {}", result);
        assert_eq!(result, 12.0);
    }
}
