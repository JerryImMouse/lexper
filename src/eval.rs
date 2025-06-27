use crate::Result;
use crate::parser::Expression;
use std::{collections::HashMap, f64};

pub struct Context {
    variables: HashMap<String, f64>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn init(&mut self) {
        self.variables.insert("PI".to_string(), f64::consts::PI);
    }

    pub fn get_var(&self, name: &str) -> Option<f64> {
        self.variables.get(name).copied()
    }

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
