use crate::types::{Expr, Statement, StatementType, Token};
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

    fn is_at_end(&self) -> bool {
        self.current == self.src.len() - 1
    }

    fn get_current<'a>(&self) -> Token {
        self.src[self.current].to_owned()
    }

    fn move_on(&mut self, step: usize) {
        if self.current + step < self.src.len() {
            self.current += step;
        }
    }

    fn match_types_vec(x: &Token, inp: &Vec<Token>) -> bool {
        for token in inp {
            if x == token {
                return true;
            }
        }
        false
    }

    ///create a list of statements from token
    //         let statements:Vec<Statement>;
    //     fn parse_to_statements(&mut self)->Vec<Statement>{
    //     while(!self.is_at_end()){
    //         statements.push(self.statement());
    //         self.move_on();
    //         }
    // statements
    //     }
    fn peek(&self, step: usize) -> Token {
        let target_index = self.current + step;
        if target_index > self.src.len() {
            Token::EOF
        } else {
            self.src[target_index].to_owned()
        }
    }

    ///validate if the next tokens (from current) match the input Vec token
    fn match_tok_in_order(&self, toks: Vec<Token>) -> bool {
        let size = toks.len();
        for i in 0..size {
            if self.peek(i) != toks[i] {
                return false;
            }
        }
        true
    }
    pub fn statement(&mut self) -> Statement {
        let tok = self.get_current();
        match tok {
            Token::Write => {
                if !self.match_tok_in_order(vec![
                    Token::Write,
                    Token::OParen,
                    Token::StringLiteral(String::from("")),
                    Token::CParen,
                    Token::SemiColon,
                ]) {
                    panic!("Invalid print statement");
                } else {
                    self.move_on(2);
                    let expr = self.expression();
                    Statement::new(expr, StatementType::ProcCall(Token::Write))
                }
            }
            _ => panic!("Invalid print statement"),
        }
    }

    // Recursive Descent Grammar
    pub fn expression(&mut self) -> Expr {
        *self.equality()
    }
    fn primary(&mut self) -> Box<Expr> {
        let x = self.get_current();
        self.move_on(1);
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
                self.move_on(1); //move out of Closed parenthesis
                return Box::new(Expr::Grouping(Box::new(expr)));
            }
            _ => panic!("Expected expression"),
        }
    }

    fn unary(&mut self) -> Box<Expr> {
        let term_tokens = vec![Token::Not, Token::Minus];

        //we consume consecutive unary

        let x = self.get_current();
        while x != Token::EOF {
            if Parser::match_types_vec(&x, &term_tokens) {
                self.move_on(1);
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

        while self.get_current() != Token::EOF {
            let x = self.get_current();
            if Parser::match_types_vec(&x, &term_tokens) {
                self.move_on(1);
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

        while self.get_current() != Token::EOF {
            let x = self.get_current();
            if Parser::match_types_vec(&x, &term_tokens) {
                self.move_on(1);
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

        while self.get_current() != Token::EOF {
            let x = self.get_current();
            if Parser::match_types_vec(&x, &term_tokens) {
                self.move_on(1);
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

        while self.get_current() != Token::EOF {
            let x = self.get_current();
            if Parser::match_types_vec(&x, &comparison_tokens) {
                self.move_on(1);
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
        while self.get_current() != Token::EOF {
            let x = self.get_current();
            if Parser::match_types_vec(&x, &vec![Token::Eq, Token::Neq]) {
                self.move_on(1);
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
