use std::fmt::Display;
#[derive(Debug)]
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

    //Builtin functions
    Write,
    Read,
    Sqrt,
    Abs,
    Sort,
    Len,
    Endl,

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
