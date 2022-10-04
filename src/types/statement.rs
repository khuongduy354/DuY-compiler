use crate::types::{Expr, Token};

#[derive(Debug)]
pub struct Statement {
    expression: Expr,
    statement_type: StatementType,
}
impl Statement {
    pub fn new(expression: Expr, statement_type: StatementType) -> Self {
        Statement {
            expression,
            statement_type,
        }
    }
}

#[derive(Debug)]
pub enum StatementType {
    If(Vec<Statement>),
    Var(Token),      //Token::identifier
    ProcCall(Token), //Token::identifier   print statement
    While(Vec<Statement>),
    For(Vec<Statement>),
}
