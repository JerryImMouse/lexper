use crate::lexer::{OperatorType, Token, TokenType};

mod expression;
pub use expression::Expression;

pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
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

    fn expect_token(&mut self, expected: TokenType) -> &Token {
        if let Some(token) = self.advance() {
            if token.ty() == &expected {
                token
            } else {
                panic!(
                    "Expected token {:?}, but found {:?}",
                    expected,
                    token.get_type()
                );
            }
        } else {
            panic!("Expected token {:?}, but found EOF", expected);
        }
    }

    pub fn parse_primary(&mut self) -> Expression {
        if let Some(t) = self.peek() {
            if let TokenType::OPERATOR(op) = t.get_type() {
                if matches!(op, OperatorType::PLUS | OperatorType::MINUS) {
                    self.advance();
                    let expr = self.parse_primary();
                    return Expression::Unary {
                        op,
                        expr: Box::new(expr),
                    };
                }
            }
        }

        if let Some(t) = self.advance() {
            match t.get_type() {
                TokenType::LITERAL(l) => Expression::Number(l),
                TokenType::LPAREN => {
                    let expr = self.parse_expression(0);
                    if let Some(j) = self.advance() {
                        match j.ty() {
                            TokenType::RPAREN => expr,
                            _ => panic!("Expected \")\""),
                        }
                    } else {
                        panic!("Expected \")\"");
                    }
                }
                TokenType::IDENTIFIER(ident) => {
                    if let Some(next) = self.peek() {
                        if let TokenType::LPAREN = next.ty() {
                            self.advance();
                            let args = self.parse_argument_list();
                            self.expect_token(TokenType::RPAREN);
                            Expression::Call {
                                callee: ident.clone(),
                                args,
                            }
                        } else {
                            Expression::Variable(ident.clone())
                        }
                    } else {
                        Expression::Variable(ident.clone())
                    }
                }
                _ => panic!("Expected number or \"(\""),
            }
        } else {
            panic!("Unexpected end of input");
        }
    }

    fn parse_argument_list(&mut self) -> Vec<Expression> {
        let mut args = Vec::new();

        if let Some(tok) = self.peek() {
            if let TokenType::RPAREN = tok.ty() {
                return args;
            }
        }

        loop {
            let expr = self.parse_expression(0);
            args.push(expr);
            match self.peek().map(|t| t.ty()) {
                Some(TokenType::COMMA) => {
                    self.advance();
                }
                Some(TokenType::RPAREN) => break,
                _ => panic!("Expected ',' or ')' in argument list"),
            }
        }

        args
    }

    pub fn parse_expression(&mut self, min_prec: u8) -> Expression {
        let mut lhs = self.parse_primary();

        while let Some(t) = self.peek() {
            if let Some(op) = t.get_op() {
                let prec = op.precendance();

                if prec < min_prec {
                    break;
                }

                self.advance();

                let rhs = self.parse_expression(prec + 1);

                lhs = Expression::Binary {
                    left: Box::new(lhs),
                    op,
                    right: Box::new(rhs),
                };
            } else {
                break;
            }
        }

        lhs
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expr_simple() {
        let raw = "2 + 3";
        let mut lexer = crate::lexer::Lexer::new(raw.to_string());
        lexer.lex();

        println!("{:#?}", lexer.tokens());

        let mut parser = Parser::new(lexer.tokens());
        let ast = parser.parse_expression(0);
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
