mod token;

pub use token::{OperatorType, Token, TokenType};

#[derive(Debug, Clone)]
pub struct Lexer {
    tokens: Vec<Token>,
    source: String,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            tokens: vec![],
            source,
        }
    }

    pub fn lex(&mut self) {
        let src = self.source.trim();

        let mut iter = src.chars().peekable();

        let mut line = 1usize;
        let mut col = 1usize;
        let mut offset = 0usize;

        while let Some(&c) = iter.peek() {
            match c {
                '0'..='9' => {
                    let local_col = col;
                    let start = offset;

                    while let Some(&d) = iter.peek() {
                        if d.is_ascii_digit() || d == '.' {
                            offset += 1;
                            col += 1;
                            iter.next();
                        } else {
                            break;
                        }
                    }

                    let slice = &self.source[start..offset];
                    let is_num = slice.parse::<f64>().expect("Error parsing number");
                    self.tokens
                        .push(Token::new(TokenType::LITERAL(is_num), line, local_col));
                }

                'a'..='z' | 'A'..='Z' | '_' => {
                    let local_col = col;
                    let start = offset;
                    while let Some(&d) = iter.peek() {
                        if d.is_alphanumeric() || d == '_' {
                            offset += 1;
                            col += 1;
                            iter.next();
                        } else {
                            break;
                        }
                    }
                    let slice = &self.source[start..offset];
                    self.tokens.push(Token::new(
                        TokenType::IDENTIFIER(slice.to_string()),
                        line,
                        local_col,
                    ));
                }

                '+' => {
                    iter.next();
                    self.tokens.push(Token::new(
                        TokenType::OPERATOR(token::OperatorType::PLUS),
                        line,
                        col,
                    ));
                    col += 1;
                    offset += 1;
                }

                '-' => {
                    iter.next();
                    self.tokens.push(Token::new(
                        TokenType::OPERATOR(token::OperatorType::MINUS),
                        line,
                        col,
                    ));
                    col += 1;
                    offset += 1;
                }

                '*' => {
                    iter.next();
                    self.tokens.push(Token::new(
                        TokenType::OPERATOR(token::OperatorType::MULTIPLY),
                        line,
                        col,
                    ));
                    col += 1;
                    offset += 1;
                }

                '/' => {
                    iter.next();
                    self.tokens.push(Token::new(
                        TokenType::OPERATOR(token::OperatorType::DIVIDE),
                        line,
                        col,
                    ));
                    col += 1;
                    offset += 1;
                }

                '%' => {
                    iter.next();
                    self.tokens.push(Token::new(
                        TokenType::OPERATOR(token::OperatorType::MODULO),
                        line,
                        col,
                    ));
                    col += 1;
                    offset += 1;
                }

                '^' => {
                    iter.next();
                    self.tokens.push(Token::new(
                        TokenType::OPERATOR(token::OperatorType::POWER),
                        line,
                        col,
                    ));
                    col += 1;
                    offset += 1;
                }

                '(' => {
                    iter.next();
                    self.tokens.push(Token::new(TokenType::LPAREN, line, col));
                    col += 1;
                    offset += 1;
                }

                ')' => {
                    iter.next();
                    self.tokens.push(Token::new(TokenType::RPAREN, line, col));
                    col += 1;
                    offset += 1;
                }

                ',' => {
                    iter.next();
                    self.tokens.push(Token::new(TokenType::COMMA, line, col));
                    col += 1;
                    offset += 1;
                }

                ' ' | '\t' => {
                    col += 1;
                    offset += 1;
                    iter.next();
                }

                '\n' => {
                    line += 1;
                    offset += 1;
                    iter.next();
                }

                c => {
                    panic!(
                        "Unknown character at: row: {}, column: {}. Char: {}",
                        line, col, c
                    );
                }
            }
        }
    }

    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_token_simple() {
        let str = "2 + 3";
        let mut lexer = Lexer::new(str.to_string());
        lexer.lex();
        let tokens = lexer.tokens();

        println!("Source string: {}", str);
        println!("{:#?}", tokens);

        assert_eq!(
            tokens,
            &vec![
                Token::new(TokenType::LITERAL(2.0), 1, 1),
                Token::new(TokenType::OPERATOR(OperatorType::PLUS), 1, 3),
                Token::new(TokenType::LITERAL(3.0), 1, 5),
            ]
        );
    }
}
