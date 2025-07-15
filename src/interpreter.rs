use crate::expressions::Statement;
use crate::evaluate::{evaluate_expression, evaluate_statement, Value};
use crate::environment::Environment;

pub struct Interpreter  {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<(), String> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, statement: Statement) -> Result<(), String> {
        evaluate_statement(statement, &mut self.env)?;
        Ok(())
    }

    fn stringify(&self, value: Value) -> String {
        match value {
            Value::Nil => "nil".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{:.0}", n)
                } else {
                    n.to_string()
                }
            },
            Value::String(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expressions::{Expr, Literal, BinaryOp};

    #[test]
    fn test_print_statement() {
        let mut interpreter = Interpreter::new();
        let statement = Statement::Print(Expr::Literal(Literal::String("Hello, World!".to_string())));
        let result = interpreter.interpret(vec![statement]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_expression_statement() {
        let mut interpreter = Interpreter::new();
        let statement = Statement::Expression(Expr::Literal(Literal::Number(42.0)));
        let result = interpreter.interpret(vec![statement]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_statements() {
        let mut interpreter = Interpreter::new();
        let statements = vec![
            Statement::Print(Expr::Literal(Literal::Number(1.0))),
            Statement::Print(Expr::Literal(Literal::Bool(true))),
            Statement::Expression(Expr::Literal(Literal::Nil)),
        ];
        let result = interpreter.interpret(statements);
        assert!(result.is_ok());
    }

    #[test]
    fn test_stringify_values() {
        let interpreter = Interpreter::new();
        
        assert_eq!(interpreter.stringify(Value::Nil), "nil");
        assert_eq!(interpreter.stringify(Value::Bool(true)), "true");
        assert_eq!(interpreter.stringify(Value::Bool(false)), "false");
        assert_eq!(interpreter.stringify(Value::Number(42.0)), "42");
        assert_eq!(interpreter.stringify(Value::Number(3.14)), "3.14");
        assert_eq!(interpreter.stringify(Value::String("hello".to_string())), "hello");
    }

    #[test]
    fn test_complex_expression() {
        let mut interpreter = Interpreter::new();
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Number(3.0))),
            op: BinaryOp::Plus,
            right: Box::new(Expr::Literal(Literal::Number(4.0))),
        };
        let statement = Statement::Print(expr);
        let result = interpreter.interpret(vec![statement]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_propagation() {
        let mut interpreter = Interpreter::new();
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::String("hello".to_string()))),
            op: BinaryOp::Minus,
            right: Box::new(Expr::Literal(Literal::Number(5.0))),
        };
        let statement = Statement::Print(expr);
        let result = interpreter.interpret(vec![statement]);
        assert!(result.is_err());
    }
}
