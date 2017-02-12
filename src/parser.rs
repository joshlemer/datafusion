use std::error::Error;
use std::fs::File;
use std::iter::Peekable;
use std::str::Chars;

use super::sql::*;

//pub enum Keyword {
//    Select,
//    From,
//    Where
//}

#[derive(Debug,Clone,PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(String),
    Operator(String),
    Number(String),
    Comma,
    Whitespace,
    Eq,
    Neq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Plus,
    Minus,
    Mult,
    Div,
    LParen,
    RParen,

    //Operator(String)
}

#[derive(Debug,Clone)]
pub enum ParserError {
    TokenizerError(String),
    ParserError(String),
}

struct Tokenizer {
    query: String,
}

impl Tokenizer {

    pub fn tokenize(&mut self) -> Result<Vec<Token>, ParserError> {

        let mut peekable = self.query.chars().peekable();

        let mut tokens : Vec<Token> = vec![];

        while let Some(token) = self.next_token(&mut peekable)? {
            tokens.push(token);
        }

        Ok(tokens.into_iter().filter(|t| match t {
            &Token::Whitespace => false,
            _ => true
        }).collect())
    }

    fn next_token(&self, chars: &mut Peekable<Chars>) -> Result<Option<Token>, ParserError> {
        match chars.peek() {
            Some(&ch) => match ch {
                // whitespace
                ' ' | '\t' | '\n' => {
                    chars.next(); // consume
                    Ok(Some(Token::Whitespace))
                },
                // identifier or keyword
                'a' ... 'z' | 'A' ... 'Z' | '_' | '@' => {
                    let mut s = String::new();
                    while let Some(&ch) = chars.peek() {
                        match ch {
                            'a' ... 'z' | 'A' ... 'Z' | '_' | '0' ... '9' => {
                                chars.next(); // consume
                                s.push(ch);
                            },
                            _ => break
                        }
                    }
                    match s.to_uppercase().as_ref() {
                        "SELECT" | "FROM" | "WHERE" | "LIMIT" | "ORDER" | "GROUP" | "BY" |
                        "UNION" | "ALL"| "UPDATE" | "DELETE" | "IN" | "NOT" | "NULL" |
                        "SET" => Ok(Some(Token::Keyword(s))),
                        _ => Ok(Some(Token::Identifier(s))),
                    }
                },
                // numbers
                '0' ... '9' => {
                    let mut s = String::new();
                    while let Some(&ch) = chars.peek() {
                        match ch {
                            '0' ... '9' => {
                                chars.next(); // consume
                                s.push(ch);
                            },
                            _ => break
                        }
                    }
                    Ok(Some(Token::Number(s)))
                },
                // operators
                '+' => { chars.next(); Ok(Some(Token::Plus)) },
                '-' => { chars.next(); Ok(Some(Token::Minus)) },
                '*' => { chars.next(); Ok(Some(Token::Mult)) },
                '/' => { chars.next(); Ok(Some(Token::Div)) },
                '=' => { chars.next(); Ok(Some(Token::Eq)) },
                '<' => {
                    chars.next(); // consume
                    match chars.peek() {
                        Some(&ch) => match ch {
                            '=' => {
                                chars.next();
                                Ok(Some(Token::LtEq))
                            },
                            '>' => {
                                chars.next();
                                Ok(Some(Token::Neq))
                            },
                            _ => Ok(Some(Token::Lt))
                        },
                        None => Ok(Some(Token::Lt))
                    }
                },
                '>' => {
                    chars.next(); // consume
                    match chars.peek() {
                        Some(&ch) => match ch {
                            '=' => {
                                chars.next();
                                Ok(Some(Token::GtEq))
                            },
                            _ => Ok(Some(Token::Gt))
                        },
                        None => Ok(Some(Token::Gt))
                    }
                },
                _ => Err(ParserError::TokenizerError(
                    String::from(format!("unhandled char '{}' in tokenizer", ch))))
            },
            None => Ok(None)
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize
}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens: tokens, index: 0 }
    }

    pub fn parse(&mut self) -> Result<ASTNode, ParserError> {
        self.parse_expr(0)
    }

    fn parse_expr(&mut self, precedence: u8) -> Result<ASTNode, ParserError> {

        let mut expr = self.parse_prefix()?;

        while let Some(tok) = self.next_token() {

            let next_precedence = self.get_precedence(&tok)?;
            if precedence >= next_precedence {
                break;
            }

            expr = self.parse_infix(expr, next_precedence)?;
        }

        Ok(expr)
    }

    fn parse_prefix(&mut self) -> Result<ASTNode, ParserError> {
        Err(ParserError::TokenizerError(String::from("not implemented yet")))
    }

    fn parse_infix(&mut self, expr: ASTNode, precedence: u8) -> Result<ASTNode, ParserError> {
        Err(ParserError::TokenizerError(String::from("not implemented yet")))
    }

    fn get_precedence(&self, tok: &Token) -> Result<u8, ParserError> {
        match tok {
            &Token::Eq | &Token::Lt | & Token::LtEq |
            &Token::Neq | &Token::Gt | & Token::GtEq => Ok(20),
            &Token::Plus | &Token::Minus => Ok(30),
            &Token::Mult | &Token::Div => Ok(40),
            _ => Err(ParserError::TokenizerError(String::from("invalid token for get_precendence")))
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.index < self.tokens.len() {
            self.index = self.index + 1;
            Some(self.tokens[self.index-1].clone())
        } else {
            None
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tokenize_select_1()  {
        let sql = String::from("SELECT 1");
        let mut tokenizer = Tokenizer { query: sql };
        let tokens = tokenizer.tokenize().unwrap();
        println!("tokens = {:?}", tokens);
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Keyword(String::from("SELECT")), tokens[0]);
        assert_eq!(Token::Number(String::from("1")), tokens[1]);
    }

    #[test]
    fn tokenize_simple_select()  {
        let sql = String::from("SELECT * FROM customer WHERE id = 1");
        let mut tokenizer = Tokenizer { query: sql };
        let tokens = tokenizer.tokenize().unwrap();
        println!("tokens = {:?}", tokens);
        assert_eq!(8, tokens.len());
        assert_eq!(Token::Keyword(String::from("SELECT")), tokens[0]);
        assert_eq!(Token::Mult, tokens[1]);
        assert_eq!(Token::Keyword(String::from("FROM")), tokens[2]);
        assert_eq!(Token::Identifier(String::from("customer")), tokens[3]);
        assert_eq!(Token::Keyword(String::from("WHERE")), tokens[4]);
        assert_eq!(Token::Identifier(String::from("id")), tokens[5]);
        assert_eq!(Token::Eq, tokens[6]);
        assert_eq!(Token::Number(String::from("1")), tokens[7]);
    }
}
