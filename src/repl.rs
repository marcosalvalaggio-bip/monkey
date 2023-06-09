use super::lexer::Lexer;
use super::token::{Token, TokenType};
use std::io::{self, Write};

const PROMPT: &str = ">> ";

pub fn start() {
    println!("Welcome to the Monkey REPL!");

    loop {
        print!("{}", PROMPT);
        io::stdout().flush().expect("Failed to flush stdout");
        // scan input line
        let stdin = io::stdin();
        let mut buffer: String = String::new();
        match stdin.read_line(&mut buffer) {
            Err(error) => {
                println!("error: {}", error);
                return;
            }
            Ok(_) => (),
        }
        let mut lexer: Lexer = Lexer::new(buffer.clone());
        let mut tok: Token = lexer.next_token();
        while tok.token_type != TokenType::Eof {
            println!("{:?}", tok.token_type);
            tok = lexer.next_token();
        }
    }       
}