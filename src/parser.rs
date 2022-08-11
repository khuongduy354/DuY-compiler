use crate::types::{Expr, Token};
// ```Java
// expression     → equality ;
// boolean        ->  equality && equality  , left associate
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → power ( ( "/" | "*" ) power )* ;
// power          -> unary ^ unary
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" ;
// ```

pub struct Parser {
    src: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(src: Vec<Token>) -> Self {
        Parser { src, current: 0 }
    }

    pub fn expression(&mut self) -> Box<Expr> {
        self.equality()
    }

    fn is_at_end(&self) -> bool {
        self.current == self.src.len()
    }

    fn get_current<'a>(&self) -> Option<Token> {
        if let Some(tok) = self.src.get(self.current).to_owned() {
            Some(tok.to_owned())
        } else {
            None
        }
    }

    fn move_on(&mut self) {
        self.current += 1;
    }

    fn match_types_vec(x: &Token, inp: &Vec<Token>) -> bool {
        for token in inp {
            if x == token {
                return true;
            }
        }
        false
    }
    // find and return content in interval (current,target)

    // Recursive Descent Grammar
    fn primary(&mut self) -> Box<Expr> {
        let x = self.get_current().expect("Expected expression");
        self.move_on();
        match x {
            Token::BooleanLiteral(_b) => Box::new(Expr::Literals(x)),
            Token::StringLiteral(_b) => Box::new(Expr::Literals(Token::StringLiteral(_b))),
            Token::IntegerLiteral(_b) => Box::new(Expr::Literals(x)),
            Token::FloatLiteral(_b) => Box::new(Expr::Literals(x)),
            Token::Identifier(_b) => Box::new(Expr::Literals(Token::Identifier(_b))),
            Token::OParen => {
                self.src
                    .iter()
                    .enumerate()
                    .position(|(index, x)| *x == Token::CParen && index >= self.current)
                    .expect("Closed parenthesis expected");
                let expr = self.expression();
                self.move_on(); //move out of Closed parenthesis
                return Box::new(Expr::Grouping(expr));
            }
            _ => panic!("Expected expression"),
        }
    }

    fn unary(&mut self) -> Box<Expr> {
        let term_tokens = vec![Token::Not, Token::Minus];

        //we consume consecutive unary
        while let Some(x) = self.get_current() {
            if Parser::match_types_vec(&x, &term_tokens) {
                self.move_on();
                let expr = self.unary();
                return Box::new(Expr::Unary((x, expr)));
            } else {
                break;
            }
        }
        //if we break the loop, means we get to highest precedence
        // which is primary
        self.primary()
    }

    fn power(&mut self) -> Box<Expr> {
        //same logic as fn equality
        let mut expr = self.unary();
        let term_tokens = vec![Token::Pow];

        while let Some(x) = self.get_current() {
            if Parser::match_types_vec(&x, &term_tokens) {
                self.move_on();
                let rhs = self.unary();
                expr = Box::new(Expr::Binary((expr, x, rhs)));
            } else {
                break;
            }
        }
        expr
    }

    fn factor(&mut self) -> Box<Expr> {
        //same logic as fn equality
        let mut expr = self.power();
        let term_tokens = vec![Token::Mul, Token::Div, Token::Mod];

        while let Some(x) = self.get_current() {
            if Parser::match_types_vec(&x, &term_tokens) {
                self.move_on();
                let rhs = self.power();
                expr = Box::new(Expr::Binary((expr, x, rhs)));
            } else {
                break;
            }
        }
        expr
    }
    fn term(&mut self) -> Box<Expr> {
        //same logic as fn equality
        let mut expr = self.factor();
        let term_tokens = vec![Token::Plus, Token::Minus];

        while let Some(x) = self.get_current() {
            if Parser::match_types_vec(&x, &term_tokens) {
                self.move_on();
                let rhs = self.factor();
                expr = Box::new(Expr::Binary((expr, x, rhs)));
            } else {
                break;
            }
        }
        expr
    }

    fn comparison(&mut self) -> Box<Expr> {
        //same logic as fn equality
        let mut expr = self.term();
        let comparison_tokens = vec![Token::Great, Token::GreatEq, Token::Less, Token::LessEq];

        while let Some(x) = self.get_current() {
            if Parser::match_types_vec(&x, &comparison_tokens) {
                self.move_on();
                let rhs = self.term();
                expr = Box::new(Expr::Binary((expr, x, rhs)));
            } else {
                break;
            }
        }
        expr
    }
    fn equality(&mut self) -> Box<Expr> {
        // for e.g a1 == a2   == a3  == a4
        // ai is an expr of higher order than equality

        let mut expr = self.comparison(); //a1

        //consuming all ai, til all == are parsed or EOF
        while let Some(x) = self.get_current() {
            if Parser::match_types_vec(&x, &vec![Token::Eq, Token::Neq]) {
                self.move_on();
                //comparison is next higher order
                let rhs = self.comparison();

                //consume into original expr
                expr = Box::new(Expr::Binary((expr, x, rhs)));
            } else {
                break;
            }
        }
        expr
    }
}
