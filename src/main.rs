mod token;
mod lexer;
mod repl;

fn main() {
    
    // without repl
    // let input: String = String::from("let x = 5;");
    // let mut lexer: lexer::Lexer = lexer::Lexer::new(input.clone());
    // let tok: token::Token = lexer.next_token();
    // println!("{}", tok.token_type);
    // println!("{}", input);
    
    // use repl
    repl::start();
}
