use core::fmt;
use std::fmt::Display;
#[derive(Debug, PartialEq)]
pub enum Token {
    // keywords
    Var,
    If,
    Then,
    Else,
    While,
    For,
    Do,
    Function,
    Begin,
    End,

    //Builtin functions
    Write,
    Read,
    Sqrt,
    Abs,
    Sort,
    Len,
    Endl,

    SemiColon, // ;
    Comment,   // // or {}
    EOF,
    WhiteSpace,
    OParen,
    CParen,
    //TODO: add array square brackets

    // operators
    Plus,    // +
    Minus,   // -
    Mul,     // *
    Assign,  // :=
    Div,     // div
    Mod,     // mod
    Pow,     // ^
    And,     // and
    Or,      // or
    Not,     // not
    Plus,   // +
    Minus,  // -
    Mul,    // *
    Assign, // :=
    Div,    // div
    Mod,    // mod

    Pow, // ^

    And, // and
    Or,  // or
    Not, // not
    Neq,     // <>
    Eq,      // =
    Less,    // <
    LessEq,  // <=
    Great,   // >
    GreatEq, // >=

    //Data Types
    Integer,
    Float,
    Boolean,
    String,
    Array(i64),

    //identifer
    Identifier(String), //function, variable names

    // literals
    StringLiteral(String), //string value
    IntegerLiteral(i64),   //number value
    FloatLiteral(f64),     //number value
    BooleanLiteral(bool),
}

impl Token {
    pub fn is_whitespace(&self) -> bool {
        match self {
            Token::WhiteSpace => true,
            _ => false,
        }
    }
}
impl Clone for Token {
    fn clone(&self) -> Token {
        match self {
            Token::Identifier(s) => Token::Identifier(s.to_string()),
            Token::StringLiteral(s) => Token::StringLiteral(s.to_string()),
            Token::IntegerLiteral(i) => Token::IntegerLiteral(*i),
            Token::FloatLiteral(f) => Token::FloatLiteral(*f),
            Token::BooleanLiteral(b) => Token::BooleanLiteral(*b),
            //account for all types of tokens
            Token::OParen => Token::OParen,
            Token::CParen => Token::CParen,
            Token::Abs => Token::Abs,
            Token::Sort => Token::Sort,
            Token::Len => Token::Len,
            Token::Endl => Token::Endl,
            Token::Write => Token::Write,
            Token::Read => Token::Read,
            Token::Sqrt => Token::Sqrt,
            Token::Var => Token::Var,
            Token::If => Token::If,
            Token::Then => Token::Then,
            Token::Else => Token::Else,
            Token::While => Token::While,
            Token::For => Token::For,
            Token::Do => Token::Do,
            Token::Function => Token::Function,
            Token::Begin => Token::Begin,
            Token::End => Token::End,
            Token::Plus => Token::Plus,
            Token::Minus => Token::Minus,
            Token::Mul => Token::Mul,
            Token::Assign => Token::Assign,
            Token::Div => Token::Div,
            Token::Mod => Token::Mod,
            Token::Pow => Token::Pow,
            Token::And => Token::And,
            Token::Or => Token::Or,
            Token::Not => Token::Not,
            Token::Neq => Token::Neq,
            Token::Eq => Token::Eq,
            Token::Less => Token::Less,
            Token::LessEq => Token::LessEq,
            Token::Great => Token::Great,
            Token::GreatEq => Token::GreatEq,
            Token::EOF => Token::EOF,
            Token::SemiColon => Token::SemiColon,
            Token::Comment => Token::Comment,
            Token::WhiteSpace => Token::WhiteSpace,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Token::Var => write!(f, "var"),
            Token::If => write!(f, "if"),
            Token::Then => write!(f, "then"),
            _ => write!(f, "{:#?}", self),
        }
    }
}
// impl PartialEq for Token {
//     fn eq(&self, other: &Token) -> bool {
//         self.to_string() == other.to_string()
//     }
// }
struct StringLiteral();
//TODO: refractor to be more precise
pub enum Expr {
    Unary((Token, Box<Expr>)),
    Binary((Box<Expr>, Token, Box<Expr>)),
    Literals(Token),
    Grouping(Box<Expr>),
}
