use tokenizer::Tokenizer;

// mod error;
mod error;
mod helper;
mod test;
mod tokenizer;
mod types;

fn main() {
    let test_str = String::from(" var abcx :=12345; {123456}");
    let mut tokenizer = Tokenizer::new(&test_str);
    let result_vec = tokenizer.tokenize_full_src().unwrap();
    result_vec.iter().for_each(|x| println!("{:?}", x));
}
