#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Special
    Illegal,
    Eof,
    // Operators
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Bang,
    Lt,
    Gt,
    Eq,
    NotEq,
    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    SemiColon,
    // Keywords
    Let,
    Function,
    True,
    False,
    If,
    Else,
    Return,
    // Identifiers and literals
    Ident,
    Int,
}

#[derive(Debug)]
pub struct Token {
    pub literal: String,
    pub token_type: TokenType,
}
