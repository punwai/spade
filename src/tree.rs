use core::error;

use anyhow::Result;

use crate::token::{self, Token, TokenType};
use crate::expressions::{BinaryOp, Expr, Literal, Statement, UnaryOp};

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

    fn consume(&mut self, types: &[TokenType], error_message: String) -> Result<Token, String>{
        if !self.match_token(types) {
            return Err(error_message);
        }
        Ok(self.previous().clone())
    }

    fn print_statement(&mut self) -> Result<Statement, String> {
        let value = match self.expression() {
            Ok(val) => val,
            Err(e) => return Err(e)
        };
        if let Err(error) = self.consume(&[TokenType::Semicolon], "Expect ';' after value.".to_string()) {
            return Err(error);
        }
        Ok(Statement::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Statement, String> {
        let value = match self.expression() {
            Ok(val) => val,
            Err(e) => return Err(e)
        };
        if let Err(error) = self.consume(&[TokenType::Semicolon], "Expect ';' after value.".to_string()) {
            return Err(error);
        }
        Ok(Statement::Expression(value))
    }

    fn var_declaration(&mut self) -> Result<Statement, String> {
        let name = self.consume(&[TokenType::Identifier], "'let' assignment must be provided a name".to_string())?;
        self.consume(&[TokenType::Equal], "'let' assignment must be followed by '='".to_string())?;

        if self.match_token(&[TokenType::Semicolon]) {
            return Ok(Statement::VarDec { 
                name: name.lexeme, 
                initializer: None
            })
        }

        let expr = self.expression()?;
        self.consume(&[TokenType::Semicolon], "Delaration must end with semicolon".to_string())?;

        return Ok(Statement::VarDec {
            name: name.lexeme,
            initializer: Some(expr),
        })
    }

    fn block_statement(&mut self) -> Result<Statement, String> {
        let statements = self.block()?;
        self.consume(&[TokenType::RightBrace], "Expect '}' after block".to_string())?;
        Ok(Statement::Block(statements))
    }

    fn if_statement(&mut self) -> Result<Statement, String> {
        self.consume(&[TokenType::LeftParen], "Expect '(' after 'if'".to_string())?;
        let condition = self.expression()?;
        self.consume(&[TokenType::RightParen], "Expect ')' after condition".to_string())?;
        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_token(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Statement::If { condition, then_branch, else_branch })
    }

    fn fn_statement(&mut self) -> Result<Statement, String> {
        let name = self.consume(&[TokenType::Identifier], "Expect function name".to_string())?;
        self.consume(&[TokenType::LeftParen], "Expect '(' after function name".to_string())?;

        let mut parameters: Vec<String> = vec![];
        while !self.is_at_end() && !self.check(TokenType::RightParen) {
            let parameter = self.consume(&[TokenType::Identifier], "Expect parameter name".to_string())?;
            parameters.push(parameter.lexeme);
            if !self.match_token(&[TokenType::Comma]) {
                break;
            }
        }
        self.consume(&[TokenType::RightParen], "Expect ')' after parameters".to_string())?;
        let body = Box::new(self.block_statement()?);
        Ok(Statement::Fn { name: name.lexeme, parameters, body })
    }

    fn return_statement(&mut self) -> Result<Statement, String> {
        if self.match_token(&[TokenType::Semicolon]) {
            Ok(Statement::Return(None))
        } else {
            let expr = self.expression()?;
            self.consume(&[TokenType::Semicolon], "Expect ';' after return value".to_string())?;
            Ok(Statement::Return(Some(expr)))
        }
    }

    fn statement(&mut self) -> Result<Statement, String> {
        if self.match_token(&[TokenType::Print]) {
            return self.print_statement();
        } else if self.match_token(&[TokenType::Let]) {
            return self.var_declaration();
        } else if self.match_token(&[TokenType::LeftBrace]) {
            return self.block_statement();
        } else if self.match_token(&[TokenType::If]) {
            return self.if_statement();
        } else if self.match_token(&[TokenType::Fn]) {  
            return self.fn_statement();
        } else if self.match_token(&[TokenType::Return]) {
            return self.return_statement();
        }

        return self.expression_statement()
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        println!("got to equality");
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
        println!("got to comparison");
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
        println!("got to term");
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
        println!("got to factor");
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
        println!("got to unary");
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

        self.call()
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
            println!("got to number match");
            if let Some(crate::token::Literal::Number(value)) = &self.previous().literal {
                return Ok(Expr::Literal(Literal::Number(*value)));
            } else {
                return Err("Number token without number literal".to_string());
            }
        }

        if self.match_token(&[TokenType::Identifier]) {
            return Ok(Expr::Literal(Literal::Var(self.previous().clone())));
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

    fn block(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements: Vec<Statement> = vec![];
        while !self.is_at_end() && !self.check(TokenType::RightBrace) {
            let stmt = self.statement()?;
            statements.push(stmt);
        }
        Ok(statements)
    }


    pub fn parse_stmt(&mut self)  -> Result<Vec<Statement>, String> {
        let mut statements: Vec<Statement> = vec![];
        while !self.is_at_end() {
            let stmt = self.statement()?;
            statements.push(stmt);
        }
        Ok(statements)
    }

    fn end_arguments(&mut self) -> Result<Vec<Expr>, String> {
        let mut arguments: Vec<Expr> = vec![];
        while !self.is_at_end() && !self.check(TokenType::RightParen) {
            let expr = self.expression()?;
            arguments.push(expr);
            if !self.match_token(&[TokenType::Comma]) {
                break;
            }
        }
        self.consume(&[TokenType::RightParen], "Expect ')' after arguments".to_string())?;
        Ok(arguments)
    }

    fn call(&mut self) -> Result<Expr, String> {
        // for functions, the callee can either be an identifier,
        // or an expression that evaluates to a function.
        let mut expr = self.primary()?;
        while self.match_token(&[TokenType::LeftParen]) {
            let arguments = self.end_arguments()?;
            expr = Expr::Call { callee: Box::new(expr), arguments };
        }
        Ok(expr)
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Expr, String> {
    let mut parser = Parser::new(tokens);
    parser.expression()
}


pub fn parse_stmt(tokens: Vec<Token>) -> Result<Vec<Statement>, String> {
    let mut parser = Parser::new(tokens);
    parser.parse_stmt()
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

    #[test]
    fn test_variable_statement() {
        let tokens = scan_tokens("let dog = 3; print dog;".to_string()).unwrap();
        let declarations = parse_stmt(tokens).unwrap();
        let ground_truth_declaration = vec![
            Statement::VarDec { name: "dog".to_string(), initializer: Some(Expr::Literal(Literal::Number(3f64))) },
            Statement::Print(
                    Expr::Literal(Literal::Var(Token {
                        token_type: crate::token::TokenType::Identifier,
                        lexeme: "dog".to_string(),
                        literal: Some(crate::token::Literal::Number(3f64)),
                        line: 1
                    }))
                )
        ];
        assert_eq!(declarations[0].to_string(), ground_truth_declaration[0].to_string());
        assert_eq!(declarations[1].to_string(), ground_truth_declaration[1].to_string());
    }
    
}
