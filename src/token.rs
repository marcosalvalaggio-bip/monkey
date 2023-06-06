use std::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident(String),  // add, foobar, x, y, ...
    Int(String),    // 123456
    Float(String),  // 123.456
    String(String), // "hello"

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
    Colon, // ':'
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


impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::ASTERISK => write!(f, "*"),
            TokenType::SLASH => write!(f, "/"),
            TokenType::PLUS => write!(f, "+"),
            TokenType::MINUS => write!(f, "-"),
            TokenType::GT => write!(f, ">"),
            TokenType::LT => write!(f, "<"),
            TokenType::EQ => write!(f, "=="),
            TokenType::NOT_EQ => write!(f, "!="),
            _ => write!(f, "{:?}", self),
        }
    }
}

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
