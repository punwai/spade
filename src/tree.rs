use crate::token::{self, Token, TokenType};
use crate::expressions::{Expr, Literal, BinaryOp, UnaryOp};

struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(*token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = match self.previous().token_type {
                TokenType::BangEqual => BinaryOp::NotEqual,
                TokenType::EqualEqual => BinaryOp::EqualEqual,
                _ => unreachable!(),
            };
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.match_token(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = match self.previous().token_type {
                TokenType::Greater => BinaryOp::Greater,
                TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                TokenType::Less => BinaryOp::Less,
                TokenType::LessEqual => BinaryOp::LessEqual,
                _ => unreachable!(),
            };
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = match self.previous().token_type {
                TokenType::Minus => BinaryOp::Minus,
                TokenType::Plus => BinaryOp::Plus,
                _ => unreachable!(),
            };
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = match self.previous().token_type {
                TokenType::Slash => BinaryOp::Divide,
                TokenType::Star => BinaryOp::Multiply,
                _ => unreachable!(),
            };
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = match self.previous().token_type {
                TokenType::Bang => UnaryOp::Not,
                TokenType::Minus => UnaryOp::Minus,
                _ => unreachable!(),
            };
            let right = self.unary()?;
            return Ok(Expr::Unary {
                op: operator,
                expr: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }

        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }

        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }

        if self.match_token(&[TokenType::Number]) {
            if let Some(crate::token::Literal::Number(value)) = &self.previous().literal {
                return Ok(Expr::Literal(Literal::Number(*value)));
            } else {
                return Err("Number token without number literal".to_string());
            }
        }

        if self.match_token(&[TokenType::String]) {
            if let Some(crate::token::Literal::String(value)) = &self.previous().literal {
                return Ok(Expr::Literal(Literal::String(value.clone())));
            } else {
                return Err("String token without string literal".to_string());
            }
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            if !self.match_token(&[TokenType::RightParen]) {
                return Err("Expect ')' after expression".to_string());
            }
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        Err("Expect expression".to_string())
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Expr, String> {
    let mut parser = Parser::new(tokens);
    parser.expression()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{scan_tokens};

    #[test]
    fn test_literal() {
        let tokens = scan_tokens("42".to_string()).unwrap();
        let expr = parse(tokens).unwrap();
        let ground_truth_expr = Expr::Literal(Literal::Number(42.0));
        assert_eq!(expr.to_string(), ground_truth_expr.to_string());
    }
    #[test]
    fn test_binary_expression() {
        let tokens = scan_tokens("1+2".to_string()).unwrap();
        let expr = parse(tokens).unwrap();
        let ground_truth_expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Number(1.0))),
            op: BinaryOp::Plus,
            right: Box::new(Expr::Literal(Literal::Number(2.0)))
        };
        assert_eq!(expr.to_string(), ground_truth_expr.to_string());
    }

    #[test]
    fn test_unary_expression() {
        let tokens = scan_tokens("-42".to_string()).unwrap();
        let expr = parse(tokens).unwrap();
        let ground_truth_expr = Expr::Unary {
            op: UnaryOp::Minus,
            expr: Box::new(Expr::Literal(Literal::Number(42.0)))
        };
        assert_eq!(expr.to_string(), ground_truth_expr.to_string());
    }
}
