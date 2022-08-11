# DuY Interpreter

An Interpreter for Pascal programming language written in Rust, that alternate the original Pascal's syntax a little bit.

# Todo

[x] tests for tokenizer
[x] parser
<!-- [] boolean opeartors -->

- Tokenizer DONE is DONE, no need to improve cuz lots of string manipulation

# Learned
- It's hard to clone an enum 
- Enum instance, Token::IntegerLiteral for example, 
only known at runtime, and cannot be seen as a type, only Token is the type

- To evaluate expression:
Expression valuated to Token type, but its type is only Token, 
cannot add or minus due to unknown type of literals, to solve do either:
1. Use Operator Overloading,     
Token + Token, match and return result  
2. Implement trait objects for i32,f64,..., then use that as a type instead of Token, 
-> CANT due to no operator for that trait objects, still need implement ops like above
-> Due to being at runtime, Rust doesnt know if it implements ops::Add 
3. Match ops in binary then match data types, pretty similar to 1, but less flexible 

- Static Inference: the tokenizer can guess the type of data, Token::IntegerLiteral or Token::FloatLiteral,
-> if allow changing data types of variables, then it's dynamic language
-> if not, then it's a static language that has inference as feature

e.g https://craftinginterpreters.com/evaluating-expressions.html#evaluating-unary-expressions
<!-- => https://doc.rust-lang.org/book/ch19-04-advanced-types.html for types handling  -->
