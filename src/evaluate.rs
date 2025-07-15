use crate::{environment::Environment, expressions::{BinaryOp, Expr, Literal, Statement, UnaryOp}};
use anyhow::Result;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
    // Later you can add:
    // Function(LoxFunction),
    // Instance(LoxInstance),
    // Class(LoxClass),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            _ => true,
        }
    }
}

pub fn evaluate_statement(stmt: Statement, env: &mut Environment) -> Result<Value, String> {
    match stmt {
        Statement::Expression(expr) => {
            evaluate_expression(expr, env)?;
            Ok(Value::Nil)
        },
        Statement::Print(expr)  => {
            let val = evaluate_expression(expr, env)?;
            println!("{:?}", val);
            Ok(Value::Nil)
        },
        Statement::Block(statements) => {
            let mut env = Environment::new_child(env);
            for statement in statements {
                evaluate_statement(statement, &mut env)?;
            }
            env.pop();
            Ok(Value::Nil)
        },
        Statement::VarDec { name, initializer } => {
            let value = match initializer {
                Some(expr) => evaluate_expression(expr, env)?,
                None => Value::Nil,
            };
            env.define(name, value);
            Ok(Value::Nil)
        },
    }
}

pub fn evaluate_expression(expr: Expr, env: &mut Environment) -> Result<Value, String> {
    match expr {
        Expr::Binary { left, op, right } => {
            let left_val = evaluate_expression(*left, env)?;
            let right_val = evaluate_expression(*right, env)?;
            evaluate_binary(left_val, op, right_val)
        },
        Expr::Unary { op, expr } => {
            let val = evaluate_expression(*expr, env)?;
            
            match op {
                UnaryOp::Minus => {
                    match val {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => Err("Invalid operand for unary -".to_string()),
                    }
                },
                UnaryOp::Not => {
                    match val {
                        Value::Bool(b) => Ok(Value::Bool(!b)),
                        Value::Nil => Ok(Value::Bool(true)),
                        _ => Ok(Value::Bool(false)),
                    }
                },
            }
        },
        Expr::Literal(literal) => Ok(literal_to_value(literal)),
        Expr::Grouping(expr) => evaluate_expression(*expr, env),
        // Expr::Variable(token) => 
        _ => unimplemented!()
    }
}

fn literal_to_value(literal: Literal) -> Value {
    match literal {
        Literal::Nil => Value::Nil,
        Literal::Bool(b) => Value::Bool(b),
        Literal::Number(n) => Value::Number(n),
        Literal::String(s) => Value::String(s),
        _ => unimplemented!()
    }
}

fn evaluate_binary(left: Value, op: BinaryOp, right: Value) -> Result<Value, String> {
    match op {
        BinaryOp::Plus => {
            match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                _ => Err("Invalid operands for +".to_string()),
            }
        },
        BinaryOp::Minus => {
            match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                _ => Err("Invalid operands for -".to_string()),
            }
        },
        BinaryOp::Multiply => {
            match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                _ => Err("Invalid operands for *".to_string()),
            }
        },
        BinaryOp::Divide => {
            match (left, right) {
                (Value::Number(l), Value::Number(r)) => {
                    if r == 0.0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(Value::Number(l / r))
                    }
                },
                _ => Err("Invalid operands for /".to_string()),
            }
        },
        _ => Err("Unsupported binary operator".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expressions::*;

    #[test]
    fn test_literal_evaluation() {
        let expr = Expr::Literal(Literal::Number(42.0));
        let mut env = Environment::new();
        let result = evaluate_expression(expr, &mut env).unwrap();
        assert_eq!(matches!(result, Value::Number(42.0)), true);

        let expr = Expr::Literal(Literal::String("hello".to_string()));
        let mut env = Environment::new();
        let result = evaluate_expression(expr, &mut env).unwrap();
        assert_eq!(matches!(result, Value::String(ref s) if s == "hello"), true);

        let expr = Expr::Literal(Literal::Bool(true));
        let mut env = Environment::new();
        let result = evaluate_expression(expr, &mut env).unwrap();
        assert_eq!(matches!(result, Value::Bool(true)), true);

        let expr = Expr::Literal(Literal::Nil);
        let mut env = Environment::new();
        let result = evaluate_expression(expr, &mut env).unwrap();
        assert_eq!(matches!(result, Value::Nil), true);
    }

    #[test]
    fn test_binary_arithmetic() {
        // Test addition
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Number(3.0))),
            op: BinaryOp::Plus,
            right: Box::new(Expr::Literal(Literal::Number(4.0))),
        };
        let mut env = Environment::new();
        let result = evaluate_expression(expr, &mut env).unwrap();
        assert_eq!(matches!(result, Value::Number(7.0)), true);

        // Test subtraction
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Number(10.0))),
            op: BinaryOp::Minus,
            right: Box::new(Expr::Literal(Literal::Number(3.0))),
        };
        let mut env = Environment::new();
        let result = evaluate_expression(expr, &mut env).unwrap();
        assert_eq!(matches!(result, Value::Number(7.0)), true);

        // Test multiplication
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Number(6.0))),
            op: BinaryOp::Multiply,
            right: Box::new(Expr::Literal(Literal::Number(7.0))),
        };
        let mut env = Environment::new();
        let result = evaluate_expression(expr, &mut env).unwrap();
        assert_eq!(matches!(result, Value::Number(42.0)), true);

        // Test division
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Number(15.0))),
            op: BinaryOp::Divide,
            right: Box::new(Expr::Literal(Literal::Number(3.0))),
        };
        let mut env = Environment::new();
        let result = evaluate_expression(expr, &mut env).unwrap();
        assert_eq!(matches!(result, Value::Number(5.0)), true);
    }
    #[test]
    fn test_division_by_zero() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Number(10.0))),
            op: BinaryOp::Divide,
            right: Box::new(Expr::Literal(Literal::Number(0.0))),
        };
        let mut env = Environment::new();
        let result = evaluate_expression(expr, &mut env);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Division by zero".to_string());
    }

    #[test]
    fn test_unary_minus() {
        let expr = Expr::Unary {
            op: UnaryOp::Minus,
            expr: Box::new(Expr::Literal(Literal::Number(42.0))),
        };
        let mut env = Environment::new();
        let result = evaluate_expression(expr, &mut env).unwrap();
        assert_eq!(matches!(result, Value::Number(-42.0)), true);
    }

    #[test]
    fn test_unary_not() {
        // Test with boolean
        let expr = Expr::Unary {
            op: UnaryOp::Not,
            expr: Box::new(Expr::Literal(Literal::Bool(true))),
        };
        let mut env = Environment::new();
        let result = evaluate_expression(expr, &mut env).unwrap();
        assert_eq!(matches!(result, Value::Bool(false)), true);

        let expr = Expr::Unary {
            op: UnaryOp::Not,
            expr: Box::new(Expr::Literal(Literal::Bool(false))),
        };
        let mut env = Environment::new();
        let result = evaluate_expression(expr, &mut env).unwrap();
        assert_eq!(matches!(result, Value::Bool(true)), true);

        // Test with nil (should return true)
        let expr = Expr::Unary {
            op: UnaryOp::Not,
            expr: Box::new(Expr::Literal(Literal::Nil)),
        };
        let mut env = Environment::new();
        let result = evaluate_expression(expr, &mut env).unwrap();
        assert_eq!(matches!(result, Value::Bool(true)), true);

        // Test with number (should return false)
        let expr = Expr::Unary {
            op: UnaryOp::Not,
            expr: Box::new(Expr::Literal(Literal::Number(42.0))),
        };
        let mut env = Environment::new();
        let result = evaluate_expression(expr, &mut env).unwrap();
        assert_eq!(matches!(result, Value::Bool(false)), true);
    }

    #[test]
    fn test_grouping() {
        let expr = Expr::Grouping(Box::new(Expr::Literal(Literal::Number(42.0))));
        let mut env = Environment::new();
        let result = evaluate_expression(expr, &mut env).unwrap();
        assert_eq!(matches!(result, Value::Number(42.0)), true);
    }

    #[test]
    fn test_invalid_operands() {
        // Test invalid operands for arithmetic
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::String("hello".to_string()))),
            op: BinaryOp::Minus,
            right: Box::new(Expr::Literal(Literal::Number(5.0))),
        };
        let mut env = Environment::new();
        let result = evaluate_expression(expr, &mut env);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid operands for -".to_string());

        // Test invalid operand for unary minus
        let expr = Expr::Unary {
            op: UnaryOp::Minus,
            expr: Box::new(Expr::Literal(Literal::String("hello".to_string()))),
        };
        let mut env = Environment::new();
        let result = evaluate_expression(expr, &mut env);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid operand for unary -".to_string());
    }

    #[test]
    fn test_complex_expression() {
        // Test (3 + 4) * 2
        let expr = Expr::Binary {
            left: Box::new(Expr::Grouping(Box::new(Expr::Binary {
                left: Box::new(Expr::Literal(Literal::Number(3.0))),
                op: BinaryOp::Plus,
                right: Box::new(Expr::Literal(Literal::Number(4.0))),
            }))),
            op: BinaryOp::Multiply,
            right: Box::new(Expr::Literal(Literal::Number(2.0))),
        };
        let mut env = Environment::new();
        let result = evaluate_expression(expr, &mut env).unwrap();
        assert_eq!(matches!(result, Value::Number(14.0)), true);
    }
}