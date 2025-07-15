use std::fmt;

use crate::token::Token;

#[derive(Clone, Copy, Debug)]
pub enum BinaryOp {
    Multiply,
    Divide,
    Plus,
    Minus,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    NotEqual,
    EqualEqual,
    And,
    Or
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOp::Multiply => write!(f, "*"),
            BinaryOp::Divide => write!(f, "/"),
            BinaryOp::Plus => write!(f, "+"),
            BinaryOp::Minus => write!(f, "-"),
            BinaryOp::Greater => write!(f, ">"),
            BinaryOp::GreaterEqual => write!(f, ">="),
            BinaryOp::Less => write!(f, "<"),
            BinaryOp::LessEqual => write!(f, "<="),
            BinaryOp::NotEqual => write!(f, "!="),
            BinaryOp::EqualEqual => write!(f, "=="),
            BinaryOp::And => write!(f, "and"),
            BinaryOp::Or => write!(f, "or"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum UnaryOp {
    Minus,
    Not 
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Minus => write!(f, "-"),
            UnaryOp::Not => write!(f, "!"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Literal {
    Nil,
    Number(f64),
    String(String),
    Bool(bool),
    Var(Token),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Nil => write!(f, "nil"),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "\"{}\"", s),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Var(b) => write!(f, "getvar {}", b.lexeme),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expr {
    Binary { left: Box<Expr>, op: BinaryOp, right: Box<Expr> },
    Unary { op: UnaryOp, expr: Box<Expr> },
    Literal(Literal),
    Grouping(Box<Expr>),
    Assign { token: Token, value: Box<Expr> },
}

#[derive(Clone, Debug)]
pub enum Statement {
    Expression(Expr),
    Print(Expr),
    Block(Vec<Statement>),
    VarDec {
        name: String,
        initializer: Option<Expr>,
    },
    If {
        condition: Expr,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary { left, op, right } => {
                write!(f, "({} {} {})", left, op, right)
            },
            Expr::Unary { op, expr } => {
                write!(f, "({}{})", op, expr)
            },
            Expr::Literal(literal) => {
                write!(f, "{}", literal)
            },
            Expr::Grouping(expr) => {
                write!(f, "(group {})", expr)
            },
            Expr::Assign { token, value } => {
                write!(f, "(assign {} {})", token.lexeme, value)
            }
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::VarDec { name, initializer } => {
                match initializer {
                    Some(expr) => write!(f, "(var {} {})", name, expr),
                    None => write!(f, "(var {})", name),
                }
            },
            Statement::Block(statements) => {
                write!(f, "(block {})", statements.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" "))
            },
            Statement::Expression(expr) => {
                write!(f, "(expr {})", expr)
            },
            Statement::Print(expr) => {
                write!(f, "(print {})", expr)
            },
            Statement::If { condition, then_branch, else_branch } => {
                write!(f, "(if {} {} {})", condition, then_branch, else_branch.as_ref().map(|b| b.to_string()).unwrap_or("".to_string()))
            },
        }
    }
}
