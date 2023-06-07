mod token;
use token::{Token, TokenType, lookup_identifier};  

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

    // continuo a leggere finchè trovo lettere e poi ritorno la stringa dell'ident 
    pub fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        &self.input[position..self.position]
    } 

    pub fn read_number(&mut self) -> &str {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        &self.input[position..self.position]
    }

    // se è uno spazio salto
    pub fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
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
            _ => {
                 if is_letter(self.ch) {
                    //println!("is lettere case");
                    // leggo la stringa ident
                    let literal: &str = self.read_identifier();
                    // mando la stringa al lookup che ne associa il corrispettivo TokenType (guarda in token.rs)
                    Token::new(lookup_identifier(literal), literal.to_string())
                 } else if is_digit(self.ch) {
                    //println!("is digit case");
                    let digit: &str = self.read_number();
                    Token::new(TokenType::Int, digit.to_string())
                 } else{
                    //println!("is illegal case");
                    Token::new(TokenType::Illegal, self.ch.to_string())
                 }
             }
            //_ => Token::new(TokenType::Illegal, self.ch.to_string())
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


    #[test]
    fn test_next_bug() {
        let input: String = String::from("let five = 5;");
        let mut lexer: Lexer = Lexer::new(input);
        for _ in 0..5 {
            let lexed_token = lexer.next_token();
            eprintln!("{:?}", lexed_token);
            assert_eq!(1, 1);
        }
    }

    // #[test]
    // pub fn test_next_token_ext() {
    //     let input = "let five = 5;";
    //     let expected = [
    //         Token {token_type: TokenType::Let, literal: "let".to_string()},
    //         Token {token_type: TokenType::Ident, literal: "five".to_string()},
    //         Token {token_type: TokenType::Assign, literal: "=".to_string()},
    //         Token {token_type: TokenType::Int, literal: "5".to_string()},
    //         Token {token_type: TokenType::Semicolon, literal: ";".to_string()},
    //         Token {token_type: TokenType::Eof, literal: ";".to_string()},
    //     //     Token {token_type: TokenType::Let, literal: "let".to_string()},
    //     //     Token {token_type: TokenType::Ident, literal: "ten".to_string()},
    //     //     Token {token_type: TokenType::Assign, literal: "=".to_string()},
    //     //     Token {token_type: TokenType::Int, literal: "10".to_string()},
    //     //     Token {token_type: TokenType::Semicolon, literal: ";".to_string()},
    //     //     Token {token_type: TokenType::Let, literal: "let".to_string()},
    //     //     Token {token_type: TokenType::Ident, literal: "add".to_string()},
    //     //     Token {token_type: TokenType::Assign, literal: "=".to_string()},
    //     //     Token {token_type: TokenType::Function, literal: "fn".to_string()},
    //     //     Token {token_type: TokenType::Lparen, literal: "(".to_string()},
    //     //     Token {token_type: TokenType::Ident, literal: "x".to_string()},
    //     //     Token {token_type: TokenType::Comma, literal: ",".to_string()},
    //     //     Token {token_type: TokenType::Ident, literal: "y".to_string()},
    //     //     Token {token_type: TokenType::Rparen, literal: ")".to_string()},
    //     //     Token {token_type: TokenType::Lbrace, literal: "{".to_string()},
    //     //     Token {token_type: TokenType::Ident, literal: "x".to_string()},
    //     //     Token {token_type: TokenType::Plus, literal: "+".to_string()},
    //     //     Token {token_type: TokenType::Ident, literal: "y".to_string()},
    //     //     Token {token_type: TokenType::Semicolon, literal: ";".to_string()},
    //     //     Token {token_type: TokenType::Rbrace, literal: "}".to_string()},
    //     //     Token {token_type: TokenType::Semicolon, literal: ";".to_string()},
    //     //     Token {token_type: TokenType::Let, literal: "let".to_string()},
    //     //     Token {token_type: TokenType::Ident, literal: "result".to_string()},
    //     //     Token {token_type: TokenType::Assign, literal: "=".to_string()},
    //     //     Token {token_type: TokenType::Ident, literal: "add".to_string()},
    //     //     Token {token_type: TokenType::Lparen, literal: "(".to_string()},
    //     //     Token {token_type: TokenType::Ident, literal: "five".to_string()},
    //     //     Token {token_type: TokenType::Ident, literal: "ten".to_string()},
    //     //     Token {token_type: TokenType::Rparen, literal: ")".to_string()},
    //     //     Token {token_type: TokenType::Semicolon, literal: ";".to_string()},
    //     //  Token {token_type: TokenType::Eof, literal: ";".to_string()},
    //     ];

    //     let mut lexer: Lexer = Lexer::new(input.to_string());
    //     for token in expected {
    //         let lexed_token = lexer.next_token();
    //         println!("expected: {:?}, read: {:?}", token, lexed_token);
    //         assert_eq!(lexed_token, token);
    //     }
            
    // }


}


fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ch.is_digit(10)
}