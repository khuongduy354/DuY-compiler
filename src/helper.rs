use crate::{error::DuYError, types::Token};

pub fn tokenize_keyword(input: &str) -> Option<Token> {
    let input = input.to_lowercase();
    match input.as_str() {
        "var" => Some(Token::Var),
        "if" => Some(Token::If),
        "then" => Some(Token::Then),
        "else" => Some(Token::Else),
        "while" => Some(Token::While),
        "for" => Some(Token::For),
        "do" => Some(Token::Do),
        "function" => Some(Token::Function),
        "begin" => Some(Token::Begin),
        "end" => Some(Token::End),
        "mod" => Some(Token::Mod),
        "and" => Some(Token::And),
        "or" => Some(Token::Or),
        "not" => Some(Token::Not),
        "write" => Some(Token::Write),
        "read" => Some(Token::Read),
        "sqrt" => Some(Token::Sqrt),
        "abs" => Some(Token::Abs),
        "sort" => Some(Token::Sort),
        "len" => Some(Token::Len),
        "endl" => Some(Token::Endl),
        "true" => Some(Token::BooleanLiteral(true)),
        "false" => Some(Token::BooleanLiteral(false)),
        _ => None,
    }
}

pub fn tokenize_ident(input: &str) -> Result<Token, DuYError> {
    //name: cant start with number, cant contain space
    if input.chars().nth(0).unwrap().is_digit(10) {
        return Err(DuYError::InvalidIdentifier(
            "Cant start with number".to_string(),
        ));
    }
    if input.contains(" ") {
        return Err(DuYError::InvalidIdentifier(
            "Cant contain space".to_string(),
        ));
    }
    let mut result = String::from("");
    for char in input.chars() {
        if char.is_alphanumeric() || char == "_".chars().nth(0).unwrap() {
            result.push(char)
        } else {
            return Err(DuYError::InvalidIdentifier(
                "Cant contain invalid character".to_string(),
            ));
        }
    }

    Ok(Token::Identifier(result))
}

pub fn tokenize_number_literals(input: &str) -> Result<Token, DuYError> {
    let mut result = String::from("");

    if input.matches(".").count() == 1 {
        for char in input.chars() {
            if char.is_digit(10) || char == ".".chars().nth(0).unwrap() {
                result.push(char)
            } else {
                return Err(DuYError::InvalidToken);
            }
        }
        return Ok(Token::FloatLiteral(result.parse::<f64>().unwrap()));
    } else if !input.contains(".") {
        for char in input.chars() {
            if char.is_digit(10) {
                result.push(char)
            } else {
                return Err(DuYError::InvalidToken);
            }
        }
        return Ok(Token::IntegerLiteral(result.parse::<i64>().unwrap()));
    } else {
        return Err(DuYError::InvalidToken);
    }
}

pub fn tokenize_string_literals(input: &str) -> Result<Token, DuYError> {
    //does not support escape characters
    if input.starts_with("'") && input.ends_with("'") {
        let mut result = String::from("");
        for _char in input.chars() {
            if _char != '\'' {
                result.push(_char);
            }
        }
        return Ok(Token::StringLiteral(result));
    }
    Err(DuYError::InvalidToken)
}

pub fn skip_comments(src: &str) -> usize {
    let pairs = [("{", "}")];

    for &(pattern, matcher) in &pairs {
        if src.starts_with(pattern) {
            let matcher = matcher.chars().nth(0).unwrap();
            for (pos, char) in src.chars().enumerate() {
                if char == matcher {
                    return pos - 1;
                }
            }
        }
    }
    0
}
