use crate::token::{Token, TokenType};

pub struct Lexer <'a>{
    input: &'a String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl<'a> Lexer <'a>{
    pub fn new(input: &'a String) -> Lexer <'a>{
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            Some('=') => {
                // could be either assign or Eq depending on next char
                match self.peek_char() {
                    Some('=') => {
                        // in this case we accept the reading of the next char
                        self.read_char();
                        Token {
                            literal: "==".to_string(),
                            token_type: TokenType::Eq,
                        }
                    },
                    _ => Token {
                        literal: "=".to_string(),
                        token_type: TokenType::Assign,
                    }
                }
            },
            Some('!') =>  {
                // could be either Bang or NotEq depending on next char
                match self.peek_char() {
                    Some('=') => {
                        // in this case we accept the reading of the next char
                        self.read_char();
                        Token {
                            literal: "!=".to_string(),
                            token_type: TokenType::NotEq,
                        }
                    },
                    _ => Token {
                        literal: "!".to_string(),
                        token_type: TokenType::Bang
                    },
                }
            },
            Some('+') => Token {
                literal: "+".to_string(),
                token_type: TokenType::Plus
            },
            Some('-') => Token {
                literal: "-".to_string(),
                token_type: TokenType::Minus
            },
            Some('*') => Token {
                literal: "*".to_string(),
                token_type: TokenType::Asterisk
            },
            Some('/') => Token {
                literal: "/".to_string(),
                token_type: TokenType::Slash
            },
            Some('<') => Token {
                literal: "<".to_string(),
                token_type: TokenType::Lt
            },
            Some('>') => Token {
                literal: ">".to_string(),
                token_type: TokenType::Gt
            },
            Some('(') => Token {
                literal: "(".to_string(),
                token_type: TokenType::LParen
            },
            Some(')') => Token {
                literal: ")".to_string(),
                token_type: TokenType::RParen
            },
            Some('{') => Token {
                literal: "{".to_string(),
                token_type: TokenType::LBrace
            },
            Some('}') => Token {
                literal: "}".to_string(),
                token_type: TokenType::RBrace
            },
            Some(',') => Token {
                literal: ",".to_string(),
                token_type: TokenType::Comma
            },
            Some(';') => Token {
                literal: ";".to_string(),
                token_type: TokenType::SemiColon
            },
            Some(c) =>
                if Lexer::is_letter(c) {
                    let literal = self.read_identifier();

                    let token_type = match literal.as_str() {
                        "let" => TokenType::Let,
                        "fn" => TokenType::Function,
                        "true" => TokenType::True,
                        "false" => TokenType::False,
                        "if" => TokenType::If,
                        "else" => TokenType::Else,
                        "return" => TokenType::Return,
                        _ => TokenType::Ident,
                    };
                    // we need to return early here because we have already
                    // read the next char and we will skip characters if we do
                    // not return early
                    return Token {
                        literal,
                        token_type,
                    }
                } else if Lexer::is_digit(c) {
                    let literal = self.read_number();

                    // we need to return early here because we have already
                    // read the next char and we will skip characters if we do
                    // not return early
                    return Token {
                        literal,
                        token_type: TokenType::Int,
                    }
                } else {
                    Token {
                        literal: c.to_string(),
                        token_type: TokenType::Illegal,
                    }
                },
            None => Token {
                literal: "".to_string(),
                token_type: TokenType::Eof
            }
        };
        self.read_char();
        token

    }

    fn read_char(&mut self) {
        self.ch = Lexer::char_at(self.input, self.read_position);
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> Option<char>{
        Lexer::char_at(self.input, self.read_position)
    }

    fn char_at(input: &String, index: usize) -> Option<char> {
        if index >= input.len() {
            None
        } else {
            input[index..index+1].chars().next()
        }
    }

    /// Note that this function effectively defines what a valid identifier is
    /// for Monkey
    fn is_letter(c: char) -> bool {
        ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || '_' == c
    }

    fn is_digit(c: char) -> bool {
        '0' <= c && c <= '9'
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;

        while let Some(c) = self.ch {
            // we read the next character, if it is a letter keep reading
            if Lexer::is_letter(c) {
                self.read_char();
            } else {
                break;
            }
        }

        self.input[start..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let start = self.position;

        while let Some(c) = self.ch {
            // we read the next character, if it is a digit keep reading
            if Lexer::is_digit(c) {
                self.read_char();
            } else {
                break;
            }
        }

        self.input[start..self.position].to_string()
    }

    fn is_whitespace(c: char) -> bool {
        c == ' ' || c == '\n' || c == '\t'
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.ch {
            if Lexer::is_whitespace(c) {
                self.read_char();
            } else {
                break;
            }
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_char() {
        let input = String::from("Hello");
        let mut lexer = Lexer::new(&input);

        assert_eq!(lexer.ch, Some('H'));
        lexer.read_char();
        assert_eq!(lexer.ch, Some('e'));
        lexer.read_char();
        assert_eq!(lexer.ch, Some('l'));
        lexer.read_char();
        assert_eq!(lexer.ch, Some('l'));
        lexer.read_char();
        assert_eq!(lexer.ch, Some('o'));
        lexer.read_char();
        assert_eq!(lexer.ch, None);
    }

    #[test]
    fn test_next_token() {
        struct Test {
            input: String,
            tokens: Vec<Token>,
        }

        let tests = [
            Test {
                input: "=+(){},;".to_string(),
                tokens: vec![
                    Token {literal: String::from("="), token_type: TokenType::Assign},
                    Token {literal: String::from("+"), token_type: TokenType::Plus},
                    Token {literal: String::from("("), token_type: TokenType::LParen},
                    Token {literal: String::from(")"), token_type: TokenType::RParen},
                    Token {literal: String::from("{"), token_type: TokenType::LBrace},
                    Token {literal: String::from("}"), token_type: TokenType::RBrace},
                    Token {literal: String::from(","), token_type: TokenType::Comma},
                    Token {literal: String::from(";"), token_type: TokenType::SemiColon},
                    Token {literal: String::from(""), token_type: TokenType::Eof},
                ]
            },
            Test {
                input: "let five = 5;".to_string(),
                tokens: vec![
                    Token {literal: String::from("let"), token_type: TokenType::Let},
                    Token {literal: String::from("five"), token_type: TokenType::Ident},
                    Token {literal: String::from("="), token_type: TokenType::Assign},
                    Token {literal: String::from("5"), token_type: TokenType::Int},
                    Token {literal: String::from(";"), token_type: TokenType::SemiColon},
                    Token {literal: String::from(""), token_type: TokenType::Eof},
                ]
            },
            Test {
                input: "let ten = 10;".to_string(),
                tokens: vec![
                    Token {literal: String::from("let"), token_type: TokenType::Let},
                    Token {literal: String::from("ten"), token_type: TokenType::Ident},
                    Token {literal: String::from("="), token_type: TokenType::Assign},
                    Token {literal: String::from("10"), token_type: TokenType::Int},
                    Token {literal: String::from(";"), token_type: TokenType::SemiColon},
                    Token {literal: String::from(""), token_type: TokenType::Eof},
                ]
            },
            Test {
                input: "let add = fn(x, y) {
                    x + y;
                };".to_string(),
                tokens: vec![
                    Token {literal: String::from("let"), token_type: TokenType::Let},
                    Token {literal: String::from("add"), token_type: TokenType::Ident},
                    Token {literal: String::from("="), token_type: TokenType::Assign},
                    Token {literal: String::from("fn"), token_type: TokenType::Function},
                    Token {literal: String::from("("), token_type: TokenType::LParen},
                    Token {literal: String::from("x"), token_type: TokenType::Ident},
                    Token {literal: String::from(","), token_type: TokenType::Comma},
                    Token {literal: String::from("y"), token_type: TokenType::Ident},
                    Token {literal: String::from(")"), token_type: TokenType::RParen},
                    Token {literal: String::from("{"), token_type: TokenType::LBrace},
                    Token {literal: String::from("x"), token_type: TokenType::Ident},
                    Token {literal: String::from("+"), token_type: TokenType::Plus},
                    Token {literal: String::from("y"), token_type: TokenType::Ident},
                    Token {literal: String::from(";"), token_type: TokenType::SemiColon},
                    Token {literal: String::from("}"), token_type: TokenType::RBrace},
                    Token {literal: String::from(";"), token_type: TokenType::SemiColon},
                    Token {literal: String::from(""), token_type: TokenType::Eof},
                ]
            },
            Test {
                input: "let result = add(five, ten);".to_string(),
                tokens: vec![
                    Token {literal: String::from("let"), token_type: TokenType::Let},
                    Token {literal: String::from("result"), token_type: TokenType::Ident},
                    Token {literal: String::from("="), token_type: TokenType::Assign},
                    Token {literal: String::from("add"), token_type: TokenType::Ident},
                    Token {literal: String::from("("), token_type: TokenType::LParen},
                    Token {literal: String::from("five"), token_type: TokenType::Ident},
                    Token {literal: String::from(","), token_type: TokenType::Comma},
                    Token {literal: String::from("ten"), token_type: TokenType::Ident},
                    Token {literal: String::from(")"), token_type: TokenType::RParen},
                    Token {literal: String::from(";"), token_type: TokenType::SemiColon},
                    Token {literal: String::from(""), token_type: TokenType::Eof},
                ],
            },
            Test {
                input: "!-/*5;".to_string(),
                tokens: vec![
                    Token {literal: String::from("!"), token_type: TokenType::Bang},
                    Token {literal: String::from("-"), token_type: TokenType::Minus},
                    Token {literal: String::from("/"), token_type: TokenType::Slash},
                    Token {literal: String::from("*"), token_type: TokenType::Asterisk},
                    Token {literal: String::from("5"), token_type: TokenType::Int},
                    Token {literal: String::from(";"), token_type: TokenType::SemiColon},
                    Token {literal: String::from(""), token_type: TokenType::Eof},
                ],
            },
            Test {
                input: "5 < 10 > 5;".to_string(),
                tokens: vec![
                    Token {literal: String::from("5"), token_type: TokenType::Int},
                    Token {literal: String::from("<"), token_type: TokenType::Lt},
                    Token {literal: String::from("10"), token_type: TokenType::Int},
                    Token {literal: String::from(">"), token_type: TokenType::Gt},
                    Token {literal: String::from("5"), token_type: TokenType::Int},
                    Token {literal: String::from(";"), token_type: TokenType::SemiColon},
                    Token {literal: String::from(""), token_type: TokenType::Eof},
                ],
            },
            Test {
                input: "if (5 < 10) {
                    return true;
                } else {
                    return false;
                }".to_string(),
                tokens: vec![
                    Token {literal: String::from("if"), token_type: TokenType::If},
                    Token {literal: String::from("("), token_type: TokenType::LParen},
                    Token {literal: String::from("5"), token_type: TokenType::Int},
                    Token {literal: String::from("<"), token_type: TokenType::Lt},
                    Token {literal: String::from("10"), token_type: TokenType::Int},
                    Token {literal: String::from(")"), token_type: TokenType::RParen},
                    Token {literal: String::from("{"), token_type: TokenType::LBrace},
                    Token {literal: String::from("return"), token_type: TokenType::Return},
                    Token {literal: String::from("true"), token_type: TokenType::True},
                    Token {literal: String::from(";"), token_type: TokenType::SemiColon},
                    Token {literal: String::from("}"), token_type: TokenType::RBrace},
                    Token {literal: String::from("else"), token_type: TokenType::Else},
                    Token {literal: String::from("{"), token_type: TokenType::LBrace},
                    Token {literal: String::from("return"), token_type: TokenType::Return},
                    Token {literal: String::from("false"), token_type: TokenType::False},
                    Token {literal: String::from(";"), token_type: TokenType::SemiColon},
                    Token {literal: String::from("}"), token_type: TokenType::RBrace},
                    Token {literal: String::from(""), token_type: TokenType::Eof}
                ],
            },
            Test {
                input: "10 == 10;".to_string(),
                tokens: vec![
                    Token {literal: "10".to_string(), token_type: TokenType::Int},
                    Token {literal: "==".to_string(), token_type: TokenType::Eq},
                    Token {literal: "10".to_string(), token_type: TokenType::Int},
                    Token {literal: ";".to_string(), token_type: TokenType::SemiColon},
                ]
            },
            Test {
                input: "10 != 5;".to_string(),
                tokens: vec![
                    Token {literal: "10".to_string(), token_type: TokenType::Int},
                    Token {literal: "!=".to_string(), token_type: TokenType::NotEq},
                    Token {literal: "5".to_string(), token_type: TokenType::Int},
                    Token {literal: ";".to_string(), token_type: TokenType::SemiColon},
                ]
            }
        ];

        for test in tests {
            let mut lexer = Lexer::new(&test.input);

            for expected_token in test.tokens {
                let token = lexer.next_token();
                assert_eq!(token.token_type, expected_token.token_type);
                assert_eq!(token.literal, expected_token.literal);
            }
        }
    }
}
