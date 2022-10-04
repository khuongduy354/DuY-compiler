use parser::Parser;
use tokenizer::Tokenizer;

// mod error;
mod error;
mod helper;
mod parser;
mod test;
mod tokenizer;
mod types;

fn main() {
    test_statements();
}

fn test_evaluation_expression() {
    let test_str = String::from("(1+2)*3+5*6+8^3/23+1312");
    // let test_str = String::from("--1");
    // let test_str = String::from("(1+2)*3");

    let mut tokenizer = Tokenizer::new(&test_str);
    let result_vec = tokenizer.tokenize_full_src().unwrap();
    // println!("{:?}", result_vec);

    let mut parser = Parser::new(result_vec);
    let expr = parser.expression();
    // print!("{}", expr);
    let evaluated = expr.eval();
    println!("{}", evaluated);
    // print!("{}", Token::While);
}
fn test_statements() {
    let test_str = "write('');";
    let mut tokenizer = Tokenizer::new(&test_str);
    let result_vec = tokenizer.tokenize_full_src().unwrap();
    println!("{:?}", result_vec);
    let mut parser = Parser::new(result_vec);
    let statement = parser.statement();
    println!("{:?}", statement);
}
