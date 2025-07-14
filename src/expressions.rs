use std::fmt;

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
    Bool(bool)
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Nil => write!(f, "nil"),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "\"{}\"", s),
            Literal::Bool(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expr {
    Binary { left: Box<Expr>, op: BinaryOp, right: Box<Expr> },
    Unary { op: UnaryOp, expr: Box<Expr> },
    Literal(Literal),
    Grouping(Box<Expr>)
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
            }
        }
    }
}
