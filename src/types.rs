use core::fmt;
use std::ops;
#[derive(Debug, PartialEq, PartialOrd)]
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
    Plus,   // +
    Minus,  // -
    Mul,    // *
    Assign, // :=
    Div,    // div
    Mod,    // mod

    Pow, // ^

    And, // and
    Or,  // or
    Not,

    Eq,
    Neq,
    Great,
    GreatEq,
    Less,
    LessEq,

    // literals
    Identifier(String),
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
            Token::EOF => Token::EOF,
            Token::SemiColon => Token::SemiColon,
            Token::Comment => Token::Comment,
            Token::WhiteSpace => Token::WhiteSpace,
            Token::Not => Token::Not,
            Token::Eq => Token::Eq,
            Token::Neq => Token::Neq,
            Token::Great => Token::Great,
            Token::GreatEq => Token::GreatEq,
            Token::Less => Token::Less,
            Token::LessEq => Token::LessEq,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Var => write!(f, "var"),
            Token::If => write!(f, "if"),
            Token::Then => write!(f, "then"),
            Token::IntegerLiteral(i) => write!(f, "{}", i),
            Token::FloatLiteral(fl) => write!(f, "{}", fl),
            Token::StringLiteral(s) => write!(f, "{}", s),
            Token::BooleanLiteral(b) => write!(f, "{}", b),
            _ => write!(f, "{:#?}", self),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Unary((Token, Box<Expr>)),
    Binary((Box<Expr>, Token, Box<Expr>)),
    Literals(Token),
    Grouping(Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Unary((t, e)) => write!(f, "<{} {}>", t, e),
            Expr::Binary((l, t, r)) => write!(f, "({} {} {})", l, t, r),
            Expr::Literals(t) => write!(f, "{}", t),
            Expr::Grouping(e) => write!(f, "({})", e),
        }
    }
}
impl Expr {
    pub fn eval(&self) -> Token {
        match self {
            Expr::Unary((ops, expr)) => match ops {
                Token::Minus => Token::IntegerLiteral(0) - expr.eval(),
                Token::Not => !expr.eval(),
                _ => panic!(),
            },

            Expr::Binary((lhs, ops, rhs)) => match ops {
                Token::Plus => lhs.eval() + rhs.eval(),
                Token::Minus => lhs.eval() - rhs.eval(),

                Token::Mul => lhs.eval() * rhs.eval(),
                Token::Div => lhs.eval() / rhs.eval(),
                Token::Mod => lhs.eval() % rhs.eval(),

                Token::Pow => lhs.eval().pow_token(rhs.eval()),

                Token::Eq => Token::BooleanLiteral(lhs.eval() == rhs.eval()),
                Token::Neq => Token::BooleanLiteral(lhs.eval() != rhs.eval()),
                Token::Great => Token::BooleanLiteral(lhs.eval() > rhs.eval()),
                Token::GreatEq => Token::BooleanLiteral(lhs.eval() >= rhs.eval()),
                Token::Less => Token::BooleanLiteral(lhs.eval() < rhs.eval()),
                Token::LessEq => Token::BooleanLiteral(lhs.eval() <= rhs.eval()),
                // Token::And => lhs.eval() && rhs.eval(),
                _ => panic!("Unsupported operator"),
            },
            Expr::Literals(value) => match value {
                Token::IntegerLiteral(i) => Token::IntegerLiteral(*i),
                Token::FloatLiteral(f) => Token::FloatLiteral(*f),
                // Token::StringLiteral(s) => s,
                Token::BooleanLiteral(b) => Token::BooleanLiteral(*b),
                _ => panic!("Unsupported literal"),
            },
            Expr::Grouping(expr) => expr.eval(),
        }
    }
}
// impl PartialEq for Token {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//
//             _ => panic!("Unsupported comparison"),
//         }
//     }
// }

//impl add for token
impl ops::Add<Token> for Token {
    type Output = Token;
    fn add(self, other: Token) -> Token {
        match (self, other) {
            (Token::IntegerLiteral(i), Token::IntegerLiteral(j)) => Token::IntegerLiteral(i + j),
            (Token::FloatLiteral(i), Token::FloatLiteral(j)) => Token::FloatLiteral(i + j),
            _ => panic!("Cannot add"),
        }
    }
}
//implement subtract for Token
impl ops::Sub<Token> for Token {
    type Output = Token;
    fn sub(self, other: Token) -> Token {
        match (self, other) {
            (Token::IntegerLiteral(i), Token::IntegerLiteral(j)) => Token::IntegerLiteral(i - j),
            (Token::FloatLiteral(i), Token::FloatLiteral(j)) => Token::FloatLiteral(i - j),
            _ => panic!("Cannot subtract"),
        }
    }
}

// implement Mul for token
impl ops::Mul<Token> for Token {
    type Output = Token;
    fn mul(self, other: Token) -> Token {
        match (self, other) {
            (Token::IntegerLiteral(i), Token::IntegerLiteral(j)) => Token::IntegerLiteral(i * j),
            (Token::FloatLiteral(i), Token::FloatLiteral(j)) => Token::FloatLiteral(i * j),
            _ => panic!("Cannot multiply"),
        }
    }
}

// implement Div for token
impl ops::Div<Token> for Token {
    type Output = Token;
    fn div(self, other: Token) -> Token {
        match (self, other) {
            (Token::IntegerLiteral(i), Token::IntegerLiteral(j)) => Token::IntegerLiteral(i / j),
            (Token::FloatLiteral(i), Token::FloatLiteral(j)) => Token::FloatLiteral(i / j),
            _ => panic!("Cannot divide"),
        }
    }
}
//implement Mod for Token
impl ops::Rem<Token> for Token {
    type Output = Token;
    fn rem(self, other: Token) -> Token {
        match (self, other) {
            (Token::IntegerLiteral(i), Token::IntegerLiteral(j)) => Token::IntegerLiteral(i % j),
            (Token::FloatLiteral(i), Token::FloatLiteral(j)) => Token::FloatLiteral(i % j),
            _ => panic!("Cannot mod"),
        }
    }
}
// implement Pow for token
pub trait Pow<Rhs = Self> {
    type Output;
    #[must_use]
    fn pow_token(self, Rhs: Self) -> Self::Output;
}

impl Pow<Token> for Token {
    type Output = Token;
    fn pow_token(self, other: Token) -> Token {
        match (self, other) {
            (Token::IntegerLiteral(i), Token::IntegerLiteral(j)) => {
                Token::IntegerLiteral(i.pow(j as u32))
            }
            (Token::FloatLiteral(i), Token::FloatLiteral(j)) => Token::FloatLiteral(i.powf(j)),
            _ => panic!("Cannot pow"),
        }
    }
}

impl ops::Not for Token {
    type Output = Token;
    fn not(self) -> Token {
        match self {
            Token::BooleanLiteral(b) => Token::BooleanLiteral(!b),
            _ => panic!("Cannot not"),
        }
    }
}

//implement and for token
// impl ops::BitAnd for Token {
//     type Output = Token;
//     fn bitand(self, other: Token) -> Token {
//         match (self, other) {
//             (Token::IntegerLiteral(i), Token::IntegerLiteral(j)) => Token::IntegerLiteral(i & j),
//             _ => panic!("Cannot and"),
//         }
//     }
// }
