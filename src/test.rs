#[cfg(test)]
mod test {

    use crate::{
        tokenizer::{self, Tokenizer},
        types::Token,
    };

    #[test]
    pub fn single_token() {
        let test_inp = vec!["+", "-", "*", ";", "^", "/", "="];
        for operator in test_inp {
            let operator = operator.to_string();
            let mut tokenizer = Tokenizer::new(&operator);
            let result = tokenizer
                .lex_next_token()
                .expect("Cannot parse single token");
            // assert!(matches!(result, Token));
        }
    }
    #[test]
    pub fn double_tokens() {
        let test_inp = vec!["<", ">", ">=", "<=", ":="];

        for operator in test_inp {
            let operator = operator.to_string();
            let mut tokenizer = Tokenizer::new(&operator);

            let result = tokenizer
                .lex_next_token()
                .expect("Cannot parse single token");
        }
    }

    #[test]
    pub fn literals() {
        let test_inp = vec!["'this is a string'", "124124", "12.4124"];

        let mut tokenizer = Tokenizer::new(test_inp[0]);
        if let Token::StringLiteral(x) = tokenizer.lex_next_token().unwrap() {
            assert_eq!(x, "this is a string");
            // println!("{}", x);
        } else {
            panic!("Cannot tokenize string literal")
        };

        let mut tokenizer = Tokenizer::new(test_inp[1]);
        if let Token::IntegerLiteral(x) = tokenizer.lex_next_token().unwrap() {
            assert_eq!(x, 124124);
            // println!("{}", x);
        } else {
            panic!("Cannot tokenize integer literal")
        };

        let mut tokenizer = Tokenizer::new(test_inp[2]);
        if let Token::FloatLiteral(x) = tokenizer.lex_next_token().unwrap() {
            assert_eq!(x, 12.4124);
            // println!("{}", x);
        } else {
            panic!("Cannot tokenize float literal")
        };
    }
}
