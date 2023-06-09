// use std::fmt;


#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident,  // add, foobar, x, y, ...
    Int,    // 123456
    //Float,  // 123.456
    //String, // "hello"

    // Operators
    Assign, // '='
    Plus, // '+'
    Minus, // '-'
    Bang, // '!'
    Asterisk, // '*'
    Slash, // '/'

    Lt, // '<'
    Gt, // '>'

    Eq, // '=='
    NotEq, // '!='

    Comma, // ','
    //Colon, // ':'
    Semicolon, // ';'

    Lparen, // '('
    Rparen, // ')'
    Lbrace, // '{'
    Rbrace, // '}'
    Lbracket, // '['
    Rbracket, // ']'

    // General Keywords
    Function, // Function declaration
    Let, // Variable declaration
    True, // true boolean
    False, // false boolean
    If, // If statement
    Else, // Else statement
    Return,  // return declaration
}


// impl fmt::Display for TokenType {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             TokenType::Asterisk => write!(f, "*"),
//             TokenType::Slash => write!(f, "/"),
//             TokenType::Plus => write!(f, "+"),
//             TokenType::Minus => write!(f, "-"),
//             TokenType::Gt => write!(f, ">"),
//             TokenType::Lt => write!(f, "<"),
//             TokenType::Eq => write!(f, "=="),
//             TokenType::NotEq => write!(f, "!="),
//             _ => write!(f, "{:?}", self),
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type: token_type,
            literal: literal,
        }
    }
}

pub fn lookup_identifier(ident: &str) -> TokenType {
    match ident {
        // General Keywords
        "let" => TokenType::Let,
        "fn" => TokenType::Function,
        "return" => TokenType::Return,

        // Conditional Statement Keywords
        "if" => TokenType::If,
        "else" => TokenType::Else,

        // Boolean Keywords
        "true" => TokenType::True,
        "false" => TokenType::False,

        // Normal identifier
        _ => TokenType::Ident,
    }
}
