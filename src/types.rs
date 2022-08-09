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
}

impl Token {
    pub fn is_whitespace(&self) -> bool {
        match self {
            Token::WhiteSpace => true,
            _ => false,
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
