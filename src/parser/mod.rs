use crate::lexer::{OperatorType, Token, TokenType};
use crate::{Error, Result};

mod expression;
pub use expression::Expression;

/// The heart struct of the lexper. It parses the token vector into a nested AST of [`Expression`]s.
pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    /// Creates a new instance of the [`Parser`], but as the argument requires a slice of tokens.
    /// This one was made to not to clone a vector of tokens after lexing.
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        self.pos += 1;
        token
    }

    fn expect_token(&mut self, expected: TokenType) -> Result<&Token> {
        if let Some(token) = self.advance() {
            if token.ty() == &expected {
                Ok(token)
            } else {
                Err(Error::unexpected_value(
                    &format!("{:?}", expected),
                    Some(&format!("{:?}", token.get_type())),
                ))
            }
        } else {
            Err(Error::unexpected_value(&format!("{:?}", expected), None))
        }
    }

    pub(crate) fn parse_primary(&mut self) -> Result<Expression> {
        if let Some(t) = self.peek() {
            if let TokenType::OPERATOR(op) = t.get_type() {
                if matches!(op, OperatorType::PLUS | OperatorType::MINUS) {
                    self.advance();
                    let expr = self.parse_primary();
                    return Ok(Expression::Unary {
                        op,
                        expr: Box::new(expr?),
                    });
                }
            }
        }

        if let Some(t) = self.advance() {
            match t.get_type() {
                TokenType::LITERAL(l) => Ok(Expression::Number(l)),
                TokenType::LPAREN => {
                    let expr = self.parse_expression(0)?;
                    if let Some(j) = self.advance() {
                        match j.ty() {
                            TokenType::RPAREN => Ok(expr),
                            _ => Err(Error::unexpected_value(")", Some(&format!("{:?}", j.ty())))),
                        }
                    } else {
                        Err(Error::unexpected_value(")", None))
                    }
                }
                TokenType::IDENTIFIER(ident) => {
                    if let Some(next) = self.peek() {
                        if let TokenType::LPAREN = next.ty() {
                            self.advance();
                            let args = self.parse_argument_list()?;
                            self.expect_token(TokenType::RPAREN)?;
                            Ok(Expression::Call {
                                callee: ident.clone(),
                                args,
                            })
                        } else {
                            Ok(Expression::Variable(ident.clone()))
                        }
                    } else {
                        Ok(Expression::Variable(ident.clone()))
                    }
                }
                _ => Err(Error::unexpected_value("number or (", None)),
            }
        } else {
            Err(Error::unexpected_value("token", None))
        }
    }

    fn parse_argument_list(&mut self) -> Result<Vec<Expression>> {
        let mut args = Vec::new();

        if let Some(tok) = self.peek() {
            if let TokenType::RPAREN = tok.ty() {
                return Ok(args);
            }
        }

        loop {
            let expr = self.parse_expression(0)?;
            args.push(expr);
            match self.peek().map(|t| t.ty()) {
                Some(TokenType::COMMA) => {
                    self.advance();
                }
                Some(TokenType::RPAREN) => break,
                _ => return Err(Error::unexpected_value(", or ( in argument list", None)),
            }
        }

        Ok(args)
    }

    /// The main method, parses the whole slice of tokens into a nested expression... recursively.
    pub fn parse_expression(&mut self, min_prec: u8) -> Result<Expression> {
        let mut lhs = self.parse_primary()?;

        while let Some(t) = self.peek() {
            if let Some(op) = t.get_op() {
                let prec = op.precendance();

                if prec < min_prec {
                    break;
                }

                self.advance();

                let rhs = self.parse_expression(prec + 1)?;

                lhs = Expression::Binary {
                    left: Box::new(lhs),
                    op,
                    right: Box::new(rhs),
                };
            } else {
                break;
            }
        }

        Ok(lhs)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expr_simple() {
        let raw = "2 + 3";
        let mut lexer = crate::lexer::Lexer::new(raw.to_string());
        lexer.lex().unwrap();

        println!("{:#?}", lexer.tokens());

        let mut parser = Parser::new(lexer.tokens());
        let ast = parser.parse_expression(0).unwrap();
        println!("{:#?}", ast);

        assert_eq!(
            ast,
            Expression::Binary {
                left: Box::new(Expression::Number(2.0)),
                op: OperatorType::PLUS,
                right: Box::new(Expression::Number(3.0))
            }
        )
    }
}
