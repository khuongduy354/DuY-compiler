// //variables, operators, if, loop, function

// enum Value {
//     String(String),
//     Number(f64),
//     Boolean(bool),
// }

// enum Scope {
//     Function,
//     Main,
// }

// struct Variable {
//     value: Value,
//     name: String,
//     scope: Scope,
// }

// pub struct Function {
//     name: String,
//     args: Vec<Variable>,
//     body: Vec<Statement>,
// }

// // pub struct If{
// // condition:
// // }

// pub enum Statement {
//     Assignment,
//     If,
//     Loop,
//     FunctionCall(Function),
// }

// pub enum Operator {
//     Add,
//     Sub,
//     Mul,
//     Div,
//     Mod,
//     Pow,
//     Eq,
//     Neq,
//     And,
//     Or,
// }
// pub enum Expression {
//     Unary {
//         operator: Operator,
//         value: Value,
//     },
//     Binary {
//         operator: Operator,
//         lhs: Value,
//         rhs: Value,
//     },
// }

#[cfg(test)]
mod test {
    use crate::{tokenizer::Tokenizer, types::Token};

    pub fn basic_token() {
        let test_str = String::from("function myname");
        let mut tokenizer = Tokenizer::new(&test_str);
        let result_vec = tokenizer.tokenize_full_src().unwrap();
    }
}
