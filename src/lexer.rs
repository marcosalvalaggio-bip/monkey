mod token;
use token::{Token, TokenType};  

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
            self.ch = mod_input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let token = match self.ch{
            '=' => Token::new(TokenType::Assign, self.ch.to_string()),
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
            '\0' => Token::new(TokenType::Eof, "".to_string()),
            _ => Token::new(TokenType::Illegal, self.ch.to_string())
        };
        self.read_char();
        token
    }
}

#[cfg(test)]
mod test {

    use super::Token;
    use super::TokenType;
    use super::Lexer;

    #[test]
    fn test_basic() {
        assert!(1 == 1);
    }

    #[test]
    pub fn test_next_token() {
        let input: String = String::from("=+(){},;");
        let expected: [Token; 8] = [ // array of Token
            Token {token_type: TokenType::Assign, literal: "=".to_string()},
            Token {token_type: TokenType::Plus, literal: "+".to_string()},
            Token {token_type: TokenType::Lparen, literal: "(".to_string()},
            Token {token_type: TokenType::Rparen, literal: ")".to_string()},
            Token {token_type: TokenType::Lbrace, literal: "{".to_string()},
            Token {token_type: TokenType::Rbrace, literal: "}".to_string()},
            Token {token_type: TokenType::Comma, literal: ",".to_string()},
            Token {token_type: TokenType::Semicolon, literal: ";".to_string()},
        ];

        let mut lexer: Lexer = Lexer::new(input);
        for token in expected {
            let lexed_token = lexer.next_token();
            assert_eq!(lexed_token, token);
        }
    }
}
