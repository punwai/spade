use anyhow::Error;

use crate::token;

#[derive(
    PartialEq,
    Eq,
    Debug, Clone, Copy
)]
pub enum TokenType {
    LeftParen, // {
    RightParen, // }
    LeftBrace, //
    RightBrace, 
    Comma,
    Dot,
    // Math
    Minus,
    Plus,
    Slash,
    Star,
    // General
    Semicolon,
    // Equality
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    String,
    Number,
    // KW
    And,
    Class,
    Else,
    False,
    Fn,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Let,
    While,
    EOF
}

// try to get reserved
pub fn match_reserved(str: &str) -> Option<TokenType> {
    let x = match str {
        "and" => TokenType::And,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,
        "for" => TokenType::For,
        "fn" => TokenType::Fn,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,
        "or" => TokenType::Or,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "true" => TokenType::True,
        "let" => TokenType::Let,
        "while" => TokenType::While,
        _ => {
            return None;
        }
    };
    return Some(x);
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
}

struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

macro_rules! ternary {
    ($condition:expr, $true_expr:expr, $false_expr:expr) => {
        if $condition { $true_expr } else { $false_expr }
    }
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_alphanumeric(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}


impl Scanner {
    pub fn new(source: String) -> Self {
        return Scanner {
            source: source,
            start: 0,
            current: 0,
            line: 1,
        };
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }


    pub fn get_token(&self, token_type: TokenType, literal: Option<Literal>) -> Token {
        let lexeme = &self.source[self.start..self.current];
        return Token {
            token_type,
            lexeme: lexeme.to_string(),
            literal: literal,
            line: self.line,
        };
    }

    pub fn get_token_simple(&self, token_type: TokenType) -> Token {
        self.get_token(token_type, None)
    }

    fn advance(&mut self) -> char {
        // panics if self.current >= len(self.source)
        let next_token = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        return next_token;
    }

    fn advance_if(&mut self, condition: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let current_char = self.source.chars().nth(self.current).unwrap();
        if current_char != condition {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn look(&self, look_ahead: usize) -> Option<char> {
        if self.current + look_ahead >= self.source.len() {
            return None;
        } 
        Some(self.source.chars().nth(self.current + look_ahead).unwrap())
    }

    fn peek(&self) -> Option<char> {
        self.look(0)
    }

    fn scan_string(&mut self) -> Result<Option<Token>, Error> {
        while let Some(t) = self.peek() {
            if t == '"' {
                break;
            }
            if t == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err(anyhow::anyhow!("Unterminated string"));
        }
        println!("string:{}", self.source[self.start + 1..self.current].to_string());
        let literal = Some(Literal::String(
            self.source[self.start + 1..self.current].to_string()
        ));
        self.advance();
        Ok(Some(self.get_token(TokenType::String, literal)))
    }

    /**
     * Either standard identifier or reserved identifier.
     */
    fn scan_identifier(&mut self) -> Token {
        println!("scanning identifier");
        while let Some(c) = self.peek() {
            if is_alphanumeric(c) {
                self.advance();
            } else {
                break;
            }
        }
        let lexeme = &self.source[self.start..self.current];
        if let Some(reserved_token) = match_reserved(lexeme) {
            return self.get_token_simple(reserved_token);
        }
        println!("scanned {}", lexeme);
        self.get_token_simple(TokenType::Identifier)
    }


    // how does the scanner work?
    // we have a thing that keeps track of the next character.
    fn scan_token(&mut self) -> Result<Option<Token>, Error> {
        // ( ) { } , . - + : * then we will add it.
        let c = self.advance();
        let next_token = match c {
            '(' => Some(self.get_token_simple(TokenType::LeftParen)),
            ')' => Some(self.get_token_simple(TokenType::RightParen)),
            '{' => Some(self.get_token_simple(TokenType::LeftBrace)),
            '}' => Some(self.get_token_simple(TokenType::RightBrace)),
            ',' => Some(self.get_token_simple(TokenType::Comma)),
            '.' => Some(self.get_token_simple(TokenType::Dot)),
            '-' => Some(self.get_token_simple(TokenType::Minus)),
            '+' => Some(self.get_token_simple(TokenType::Plus)),
            ';' => Some(self.get_token_simple(TokenType::Semicolon)),
            '*' => Some(self.get_token_simple(TokenType::Star)),
            '!' => {
                let token_type = ternary!(self.advance_if('='), TokenType::BangEqual, TokenType::Bang);
                Some(self.get_token_simple(token_type))
            },
            '=' => {
                let token_type = ternary!(self.advance_if('='), TokenType::EqualEqual, TokenType::Equal);
                Some(self.get_token_simple(token_type))
            },
            '<' => {
                let token_type = ternary!(self.advance_if('='), TokenType::LessEqual, TokenType::Less);
                Some(self.get_token_simple(token_type))
            },
            '>' => {
                let token_type = ternary!(self.advance_if('='), TokenType::GreaterEqual, TokenType::Greater);
                Some(self.get_token_simple(token_type))
            },
            '/' => {
                if self.advance_if('/') {
                    while !self.peek().is_none() && self.peek() != Some('\n') {
                        self.advance();
                    }
                    None
                } else {
                    Some(self.get_token_simple(TokenType::Slash))
                }
            },
            ' ' | '\r' | '\t' => None ,
            '\n' => {
                self.line += 1;
                None
            },
            '"' => {
                return self.scan_string()
            },
            _ => {
                if is_digit(c) {
                    return Ok(self.scan_number());
                }
                if is_alpha(c) {
                    return Ok(Some(self.scan_identifier()));
                }
                return Err(anyhow::anyhow!("Unexpected character: {}", c))
            }
        };
        return Ok(next_token);
    }

    // Go through the source and scan it one by one.
    fn scan_tokens(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = vec![];

        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(token) => {
                    if let Some(t) = token {
                        tokens.push(t)
                    }
                }
                Err(e) => return Err(e)
            }
        }
        Ok(tokens)
    }

    // scan number and pro
    fn scan_number(&mut self) -> Option<Token> {
        while let Some(c) = self.peek() {
            if !is_digit(c) {
                break;
            }
            self.advance();
        }

        // If the '.' is valid, we continue to decode it.
        if self.peek() == Some('.') {
            if let Some(c) = self.look(1) {
                if is_digit(c) {
                    self.advance();
                    while let Some(c) = self.peek() {
                        if !is_digit(c) {
                            break;
                        }
                        self.advance();
                    }
                }
            }
        }

        Some(self.get_token(TokenType::Number, Some(Literal::Number(
            self.source[self.start..self.current].parse::<f64>().unwrap()
        ))))
    }
}

pub fn scan_tokens(source: String) -> Result<Vec<Token>, Error> {
    return Scanner::new(source).scan_tokens();
}

#[cfg(test)]
mod tests {
    use std::string;

    use super::*;
    
    fn match_types(tokens: Vec<Token>, types: Vec<TokenType>) {
        for (token, ttype) in tokens.iter().zip(types.iter()) {
            assert_eq!(token.token_type, *ttype);
        }
    }

    #[test]
    fn test_scan_simple_tokens() {
        let source = "(){},.+-;*".to_string();
        let expected_types = vec![
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::RightBrace,
            TokenType::Comma,
            TokenType::Dot,
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Semicolon,
            TokenType::Star
        ];
        let tokens = scan_tokens(source).unwrap();
        match_types(tokens, expected_types)
    }

    #[test]
    fn test_multiple_tokens() {
        let source = "! != == = > >= < <=".to_string();
        let expected_types = vec![
            TokenType::Bang,
            TokenType::BangEqual,
            TokenType::EqualEqual,
            TokenType::Equal,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual
        ];
        let tokens = scan_tokens(source).unwrap();
        match_types(tokens, expected_types)
    }

    #[test]
    fn test_string() {
        let source = "\"hello world\"".to_string();
        let string_token = &scan_tokens(source).unwrap()[0];
        assert_eq!(string_token.token_type, TokenType::String);
        assert_eq!(string_token.literal, Some(Literal::String("hello world".to_string())));
    }

    #[test]
    fn test_number() {
        let source: String = "34.33".to_string();
        let string_token = &scan_tokens(source).unwrap()[0];
        assert_eq!(string_token.literal, Some(Literal::Number(34.33f64)));
    }

    #[test]
    fn test_arithmetic() {
        let source: String = "3+4".to_string();
        let string_token = &scan_tokens(source).unwrap();
    }

    #[test]
    fn test_identifier() {
        let source: String = "abcd".to_string();
        let string_token = &scan_tokens(source).unwrap()[0];
        assert_eq!(string_token.token_type, TokenType::Identifier);
        assert_eq!(string_token.lexeme, "abcd");

        let source: String = "orchid".to_string();
        let string_token = &scan_tokens(source).unwrap()[0];
        assert_eq!(string_token.token_type, TokenType::Identifier);
        assert_eq!(string_token.lexeme, "orchid");

        let source: String = "or".to_string();
        let string_token = &scan_tokens(source).unwrap()[0];
        assert_eq!(string_token.token_type, TokenType::Or);
    }
}