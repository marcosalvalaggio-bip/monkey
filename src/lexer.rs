use super::token::{Token, TokenType, lookup_identifier};

#[derive(Debug)]
pub struct Lexer {
    pub input: String, 
    pub position: usize, // current position in input (points to current char)
    pub read_position: usize, // current reading position in input (after current char)
    pub ch: char, // current char under examination
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

    pub fn peek_char(&self) -> char {
        let mod_input: Vec<char> = self.input.chars().collect();
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            mod_input[self.read_position]
        }
    }

    // continuo a leggere finchè trovo lettere e poi ritorno la stringa dell'ident 
    pub fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        // eprintln!("position: {:?}", self.position);
        // eprintln!("read_position: {:?}", self.read_position);
        // eprintln!("ch: {:?}", self.ch);
        self.read_position -=1;
        &self.input[position..self.position]
    } 

    pub fn read_number(&mut self) -> &str {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        // eprintln!("position: {:?}", self.position);
        // eprintln!("read_position: {:?}", self.read_position);
        // eprintln!("ch: {:?}", self.ch);
        // se non metto il read_position -1 quando esco dal primo match salta il simbilo successivo
        self.read_position -=1;
        &self.input[position..self.position]
    }

    // se è uno spazio salto
    pub fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch{
            // '=' => Token::new(TokenType::Assign, self.ch.to_string()),
            //'!' => Token::new(TokenType::Bang, self.ch.to_string()),
            '=' => {
                if self.peek_char() == '=' {
                    let ch: String = String::from(self.ch);
                    self.read_char();
                    Token::new(TokenType::Eq, ch + &self.ch.to_string())
                } else {
                    Token::new(TokenType::Assign, self.ch.to_string()) 
                }
            },
            '!' => {
                if self.peek_char() == '=' {
                    let ch: String = String::from(self.ch);
                    self.read_char();
                    Token::new(TokenType::NotEq, ch + &self.ch.to_string())
                } else {
                    Token::new(TokenType::Bang, self.ch.to_string())
                }
            },
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
            '>' => Token::new(TokenType::Gt, self.ch.to_string()),
            '<' => Token::new(TokenType::Lt, self.ch.to_string()),
            '*' => Token::new(TokenType::Asterisk, self.ch.to_string()),
            '/' => Token::new(TokenType::Slash, self.ch.to_string()),
            '\0' => Token::new(TokenType::Eof, "".to_string()),
            _ => {
                 if is_letter(self.ch) {
                    // leggo la stringa ident
                    let literal: &str = self.read_identifier();
                    // mando la stringa al lookup che ne associa il corrispettivo TokenType (guarda in token.rs)
                    Token::new(lookup_identifier(literal), literal.to_string())
                 } else if is_digit(self.ch) {
                    let digit: &str = self.read_number();
                    Token::new(TokenType::Int, digit.to_string())
                 } else {
                    Token::new(TokenType::Illegal, self.ch.to_string())
                 }
             }
        };
        self.read_char();
        token
    }
    
}


fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ch.is_digit(10)
}

#[cfg(test)]
mod test {

    use super::Lexer;
    use super::super::token::{TokenType, Token};

    #[test]
    fn test_next_token() {
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
            //eprintln!("token: {:?}", lexed_token);
            assert_eq!(lexed_token, token);
        }
    }    


    #[test]
    fn test_next_token_ext() {
        let input = "let five = 5;";
        let expected = [
            Token {token_type: TokenType::Let, literal: "let".to_string()},
            Token {token_type: TokenType::Ident, literal: "five".to_string()},
            Token {token_type: TokenType::Assign, literal: "=".to_string()},
            Token {token_type: TokenType::Int, literal: "5".to_string()},
            Token {token_type: TokenType::Semicolon, literal: ";".to_string()},
        ];
        let mut lexer: Lexer = Lexer::new(input.to_string());
        for token in expected {
            let lexed_token = lexer.next_token();
            //println!("expected: {:?}, read: {:?}", token, lexed_token);
            assert_eq!(lexed_token, token);
        }
            
    }

    #[test]
    fn test_next_token_ext_2() {
        let input = "
        let five = 5;
        let ten = 10;

        let add = fn(x, y){
            x+y;
        };
        
        let result = add(five, ten);
        ";
        let expected = [
            Token {token_type: TokenType::Let, literal: "let".to_string()},
            Token {token_type: TokenType::Ident, literal: "five".to_string()},
            Token {token_type: TokenType::Assign, literal: "=".to_string()},
            Token {token_type: TokenType::Int, literal: "5".to_string()},
            Token {token_type: TokenType::Semicolon, literal: ";".to_string()},
            Token {token_type: TokenType::Let, literal: "let".to_string()},
            Token {token_type: TokenType::Ident, literal: "ten".to_string()},
            Token {token_type: TokenType::Assign, literal: "=".to_string()},
            Token {token_type: TokenType::Int, literal: "10".to_string()},
            Token {token_type: TokenType::Semicolon, literal: ";".to_string()},
            Token {token_type: TokenType::Let, literal: "let".to_string()},
            Token {token_type: TokenType::Ident, literal: "add".to_string()},
            Token {token_type: TokenType::Assign, literal: "=".to_string()},
            Token {token_type: TokenType::Function, literal: "fn".to_string()},
            Token {token_type: TokenType::Lparen, literal: "(".to_string()},
            Token {token_type: TokenType::Ident, literal: "x".to_string()},
            Token {token_type: TokenType::Comma, literal: ",".to_string()},
            Token {token_type: TokenType::Ident, literal: "y".to_string()},
            Token {token_type: TokenType::Rparen, literal: ")".to_string()},
            Token {token_type: TokenType::Lbrace, literal: "{".to_string()},
            Token {token_type: TokenType::Ident, literal: "x".to_string()},
            Token {token_type: TokenType::Plus, literal: "+".to_string()},
            Token {token_type: TokenType::Ident, literal: "y".to_string()},
            Token {token_type: TokenType::Semicolon, literal: ";".to_string()},
            Token {token_type: TokenType::Rbrace, literal: "}".to_string()},
            Token {token_type: TokenType::Semicolon, literal: ";".to_string()},
            Token {token_type: TokenType::Let, literal: "let".to_string()},
            Token {token_type: TokenType::Ident, literal: "result".to_string()},
            Token {token_type: TokenType::Assign, literal: "=".to_string()},
            Token {token_type: TokenType::Ident, literal: "add".to_string()},
            Token {token_type: TokenType::Lparen, literal: "(".to_string()},
            Token {token_type: TokenType::Ident, literal: "five".to_string()},
            Token {token_type: TokenType::Comma, literal: ",".to_string()},
            Token {token_type: TokenType::Ident, literal: "ten".to_string()},
            Token {token_type: TokenType::Rparen, literal: ")".to_string()},
            Token {token_type: TokenType::Semicolon, literal: ";".to_string()},
            Token {token_type: TokenType::Eof, literal: "".to_string()}
        ];

        let mut lexer: Lexer = Lexer::new(input.to_string());
        for token in expected {
            let lexed_token = lexer.next_token();
            //println!("expected: {:?}, read: {:?}", token, lexed_token);
            assert_eq!(lexed_token, token);
        }
            
    }


    #[test]
    fn extended_tokens() {
        let input = "
            !-/*5;
            5 < 10 > 5;
        ";
        let expected = [
            Token::new(TokenType::Bang, "!".to_string()),
            Token::new(TokenType::Minus, "-".to_string()),
            Token::new(TokenType::Slash, "/".to_string()),
            Token::new(TokenType::Asterisk, "*".to_string()),
            Token::new(TokenType::Int, "5".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Int, "5".to_string()),
            Token::new(TokenType::Lt, "<".to_string()),
            Token::new(TokenType::Int, "10".to_string()),
            Token::new(TokenType::Gt, ">".to_string()),
            Token::new(TokenType::Int, "5".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];
        let mut lexer = Lexer::new(input.to_string());
        for token in expected.iter() {
            let lexed_token = lexer.next_token();
            assert_eq!(lexed_token, *token);
        }
    }


    #[test]
    fn if_else_tokens() {
        let input = "
            if (5 < 10) {
                return true;
            } else {
                return false;
            }
        ";
        let expected = [
            Token::new(TokenType::If, "if".to_string()),
            Token::new(TokenType::Lparen, "(".to_string()),
            Token::new(TokenType::Int, "5".to_string()),
            Token::new(TokenType::Lt, "<".to_string()),
            Token::new(TokenType::Int, "10".to_string()),
            Token::new(TokenType::Rparen, ")".to_string()),
            Token::new(TokenType::Lbrace, "{".to_string()),
            Token::new(TokenType::Return, "return".to_string()),
            Token::new(TokenType::True, "true".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Rbrace, "}".to_string()),
            Token::new(TokenType::Else, "else".to_string()),
            Token::new(TokenType::Lbrace, "{".to_string()),
            Token::new(TokenType::Return, "return".to_string()),
            Token::new(TokenType::False, "false".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Rbrace, "}".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];

        let mut lexer = Lexer::new(input.to_string());

        for token in expected.iter() {
            let lexed_token = lexer.next_token();
            assert_eq!(lexed_token, *token);
        }
    }

    #[test]
    fn equals_not_equals() {
        let input = "
            10 == 10;
            10 != 9;
        ";

        let expected = [
            Token::new(TokenType::Int, "10".to_string()),
            Token::new(TokenType::Eq, "==".to_string()),
            Token::new(TokenType::Int, "10".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Int, "10".to_string()),
            Token::new(TokenType::NotEq, "!=".to_string()),
            Token::new(TokenType::Int, "9".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];

        let mut lexer = Lexer::new(input.to_string());

        for token in expected.iter() {
            let lexed_token = lexer.next_token();
            assert_eq!(lexed_token, *token);
        }
    }

}
