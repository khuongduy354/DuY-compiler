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
    let test_str = String::from("(1+2)*3");

    let mut tokenizer = Tokenizer::new(&test_str);
    let result_vec = tokenizer.tokenize_full_src().unwrap();
    // println!("{:?}", result_vec);

    let mut parser = Parser::new(result_vec);
    let expr = parser.expression();
    println!("{}", expr);
}
