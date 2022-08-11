use core::fmt;

use super::token::Token;
use crate::types::Pow;

#[derive(Debug)]
pub enum Expr {
    Unary((Token, Box<Expr>)),
    Binary((Box<Expr>, Token, Box<Expr>)),
    Literals(Token),
    Grouping(Box<Expr>),
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
