use std::error::Error;
use std::io::Write;
use std::{env, io};

use crate::lexer::Lexer;
use crate::token::TokenType;

const PROMPT: &str = ">>";

pub fn start() -> Result<(), Box<dyn Error>> {
    let user = env::var("USER").unwrap_or("Monkey".to_string());
    println!("Hello, {}!", user);

    loop {
        print!("{} ", PROMPT);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let mut lexer = Lexer::new(input);
        let mut token = lexer.next_token();

        while token.token_type != TokenType::Eof {
            println!("{:?}", token);
            token = lexer.next_token();
        }
        println!();
    }
}
