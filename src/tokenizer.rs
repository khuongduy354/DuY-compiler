use crate::helper::{self, skip_comments, tokenize_keyword, tokenize_string_literals};

use crate::error::DuYError;
use crate::types::Token;

pub struct Tokenizer {
    pos: usize,
    current_char: char,
    src: Vec<char>,
}
impl Tokenizer {
    pub fn new(src: &str) -> Self {
        let src: Vec<char> = src.chars().collect();
        Tokenizer {
            pos: 0,
            current_char: src[0],
            src,
        }
    }
    pub fn pos_over_end(&self, pos: usize) -> bool {
        pos >= self.src.len()
    }
    pub fn lex_next_token(&mut self) -> Result<Token, DuYError> {
        let tok: Token;
        if self.pos_over_end(self.pos) {
            return Ok(Token::EOF);
        }
        match self.current_char {
            '+' => tok = Token::Plus,
            '-' => tok = Token::Minus,
            '*' => tok = Token::Mul,
            ';' => tok = Token::SemiColon,
            '^' => tok = Token::Pow,
            '!' => tok = Token::Not,
            '|' => tok = Token::Or,
            '&' => tok = Token::And,
            '=' => tok = Token::Eq,
            '/' => tok = Token::Div,

            '^' => tok = Token::Pow,
            ';' => tok = Token::SemiColon,
            '(' => tok = Token::OParen,
            ')' => tok = Token::CParen,

            // multiple tokens
            '<' => match self.look_ahead(1) {
                Some('=') => tok = Token::LessEq,
                Some('>') => tok = Token::Neq,
                _ => tok = Token::Less,
            },
            '>' => match self.look_ahead(1) {
                Some('=') => tok = Token::GreatEq,
                _ => tok = Token::Great,
            },
            ':' => match self.look_ahead(1) {
                Some('=') => {
                    tok = Token::Assign;
                    self.move_on(1);
                }
                _ => return Err(DuYError::InvalidToken),
            },
            //string literal
            '\'' => {
                let final_pos = self.find_string_colon(self.pos + 1, '\'');
                if let Some(final_pos) = final_pos {
                    let input = &self.src[self.pos..final_pos + 1]
                        .into_iter()
                        .collect::<String>();
                    tok = tokenize_string_literals(input)?;
                    self.move_on(input.len() - 1);
                } else {
                    return Err(DuYError::InvalidToken);
                }
            }
            //number literal
            '0'..='9' => {
                //temporary, to hold number literal
                let mut temp = self.current_char.to_string();

                // look next position
                let mut look_ahead = self.look_ahead(temp.len());

                //While not EOF or is number => push to words
                while let Some(next) = look_ahead {
                    if next.is_numeric() || next == '.' {
                        temp.push(next);
                        look_ahead = self.look_ahead(temp.len());
                    } else {
                        break;
                    }
                }

                //create token from that temp
                tok = helper::tokenize_number_literals(&temp)?;
                self.move_on(temp.len() - 1);
            }

            //identifier or keyword
            c if c.is_alphanumeric() || c == '_' => {
                //placeholder for indentifier or keyword
                let mut temp = self.current_char.to_string();
                // look into next position
                let mut look_ahead = self.look_ahead(temp.len());

                //While not EOF or is letters => push to words
                while let Some(next) = look_ahead {
                    if next.is_alphanumeric() || next == '_' {
                        temp.push(next);
                        look_ahead = self.look_ahead(temp.len());
                    } else {
                        break;
                    }
                }
                //create token from that temp
                if temp == "true" {
                    tok = Token::BooleanLiteral(true)
                } else if temp == "false" {
                    tok = Token::BooleanLiteral(false)
                } else if let Some(_tok) = tokenize_keyword(&temp) {
                    tok = _tok;
                } else {
                    tok = helper::tokenize_ident(&temp)?;
                }

                self.move_on(temp.len() - 1);
            }

            '{' => {
                let steps_to_skip =
                    skip_comments(&self.src[self.pos..].into_iter().collect::<String>());
                if steps_to_skip == 0 {
                    return Err(DuYError::InvalidToken);
                } else {
                    self.move_on(steps_to_skip + 1);
                    tok = Token::Comment;
                }
            }
            a if a.is_whitespace() => tok = Token::WhiteSpace,

            _ => return Err(DuYError::InvalidToken),
        }
        self.move_on(1);
        Ok(tok)
    }

    ///move to next non whitespace char
    pub fn move_on(&mut self, step: usize) {
        //move to next non whitespace
        let mut temp_pos = self.pos + step;
        if self.pos_over_end(temp_pos) {
            self.pos = self.src.len();
            return;
        }

        while self.src[temp_pos].is_whitespace() {
            temp_pos += 1;
        }
        if !self.pos_over_end(temp_pos) {
            self.pos = temp_pos;
            self.current_char = self.src[self.pos];
        }
    }

    /// return char in the next n position
    pub fn look_ahead(&mut self, n: usize) -> Option<char> {
        let ahead_pos = self.pos + n;
        if self.pos_over_end(ahead_pos) {
            return None;
        }

        Some(self.src[ahead_pos])
    }

    pub fn next_white_space(&mut self) -> usize {
        //TODO: refractor
        let mut pos = self.pos + 1;
        while !self.pos_over_end(pos) && !self.src[pos].is_whitespace() {
            pos += 1;
        }
        if !self.pos_over_end(pos) {
            pos
        } else {
            0
        }
    }
    pub fn tokenize_full_src(&mut self) -> Result<Vec<Token>, DuYError> {
        let mut result: Vec<Token> = vec![];
        while !self.pos_over_end(self.pos) {
            let tok = self.lex_next_token()?;
            if !tok.is_whitespace() {
                result.push(tok);
            }
        }
result.push(Token::EOF); 
        Ok(result)
    }

    /// return position of first matched pattern, if cant find return None
    pub fn find_string_colon(&self, start: usize, pattern: char) -> Option<usize> {
        let mut temp_pos = start;
        while self.src[temp_pos] != pattern && !temp_pos >= self.src.len() {
            //escape the \t \n \\ \'
            if self.src[temp_pos] == '\\' {
                match self.src[temp_pos + 1] {
                    '\'' | '\\' | 't' | 'n' => temp_pos += 2,
                    _ => return None,
                }
            } else {
                temp_pos += 1;
            }
        }
        if temp_pos >= self.src.len() {
            None
        } else {
            Some(temp_pos)
        }
    }
}
