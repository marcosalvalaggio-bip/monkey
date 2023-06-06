mod token;
use token::Token;  

#[derive(Debug)]
pub struct Lexer {
    input: String, 
    position: usize, // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: char, // current char under examination
}


impl Lexer {

    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input.clone(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        let mod_input: Vec<char> = self.input.chars().collect();
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = &mod_input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let token = match self.ch{
            '=' => Token::new(TokenType::Plus, self.ch.to_string()),
            ';' => Token::new(TokenType::Semicolon, self.ch.to_string()),
            '(' => Token::new(TokenType::Lparen, self.ch.to_string()),
            ')' => Token::new(TokenType::Rparen, self.ch.to_string()),
            ',' => Token::new(TokenType::Comma, self.ch.to_string()),
            '+' => Token::new(TokenType::Plus, self.ch.to_string()),
            '-' => Token::new(TokenType::Minus, self.ch.to_string()),
            '{' => Token::new(TokenType::Lbrace, self.ch.to_string()),
            '}' => Token::new(TokenType::Rbrace, self.ch.to_string()),
            '[' => Token::new(TokenType::Lbracket, self.ch.to_string()),
            ']' => Token::new(TokenType::Rbracket, self.ch.to_string()),
            '\0' => Token::new(TokenType::EOF, "".to_string()),
            _ => Token::new(TokenType::ILLEGAL, self.current_char.to_string())
        };
        self.read_char();
        token
    }


}