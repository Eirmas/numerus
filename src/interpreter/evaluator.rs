use super::{Environment, Value};
use crate::error::NumerusError;
use crate::parser::*;
use crate::roman::to_roman;

/// The Numerus++ interpreter
pub struct Interpreter {
    env: Environment,
    output: Vec<String>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            output: Vec::new(),
        }
    }

    /// Run a program and return collected output
    pub fn run(&mut self, program: &Program) -> Result<Vec<String>, NumerusError> {
        self.output.clear();

        for statement in &program.statements {
            self.execute_statement(statement)?;
        }

        Ok(self.output.clone())
    }

    /// Execute a single statement (for REPL mode)
    pub fn execute(&mut self, statement: &Statement) -> Result<Option<String>, NumerusError> {
        self.output.clear();
        self.execute_statement(statement)?;
        Ok(self.output.pop())
    }

    /// Execute a statement
    fn execute_statement(&mut self, stmt: &Statement) -> Result<(), NumerusError> {
        match stmt {
            Statement::Declaration { name, value, .. } => {
                let val = self.evaluate_expression(value)?;
                self.env.declare(name.clone(), val)?;
            }

            Statement::Assignment { name, value, .. } => {
                let val = self.evaluate_expression(value)?;
                self.env.assign(name, val)?;
            }

            Statement::Print { value, .. } => {
                let val = self.evaluate_expression(value)?;
                let output = val.to_output_string()?;
                println!("{}", output);
                self.output.push(output);
            }

            Statement::Avtem { .. } => {
                // AVTEM - The ceremonial no-op
                // In the spirit of Roman grandeur, this does absolutely nothing
                // but adds tremendous swagger to your code
            }

            Statement::Comment { .. } => {
                // Comments are for the historians, not the executor
            }
        }
        Ok(())
    }

    /// Evaluate an expression to a Value
    fn evaluate_expression(&self, expr: &Expression) -> Result<Value, NumerusError> {
        match expr {
            Expression::NumberLiteral { value, .. } => Ok(Value::Number(*value)),

            Expression::StringLiteral { value, .. } => Ok(Value::String(value.clone())),

            Expression::Variable { name, .. } => self.env.get(name),

            Expression::BinaryOp { left, operator, right, span } => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;

                match operator {
                    BinaryOperator::Add => {
                        // ADDIUS works for both numbers and strings (concatenation)
                        match (&l, &r) {
                            (Value::Number(a), Value::Number(b)) => {
                                a.checked_add(*b)
                                    .map(Value::Number)
                                    .ok_or(NumerusError::IntegerOverflow {
                                        value: *a as i64 + *b as i64,
                                    })
                            }
                            (Value::String(a), Value::String(b)) => {
                                Ok(Value::String(format!("{}{}", a, b)))
                            }
                            (Value::String(a), Value::Number(b)) => {
                                // String + Number: convert number to string (Roman by default)
                                let num_str = to_roman(*b).unwrap_or_else(|_| b.to_string());
                                Ok(Value::String(format!("{}{}", a, num_str)))
                            }
                            (Value::Number(a), Value::String(b)) => {
                                // Number + String: convert number to string (Roman by default)
                                let num_str = to_roman(*a).unwrap_or_else(|_| a.to_string());
                                Ok(Value::String(format!("{}{}", num_str, b)))
                            }
                        }
                    }
                    BinaryOperator::Subtract => {
                        match (&l, &r) {
                            (Value::Number(a), Value::Number(b)) => {
                                a.checked_sub(*b)
                                    .map(Value::Number)
                                    .ok_or(NumerusError::IntegerOverflow {
                                        value: *a as i64 - *b as i64,
                                    })
                            }
                            _ => Err(NumerusError::TypeMismatch {
                                operation: "SUBTRAHE".to_string(),
                                expected: "numbers".to_string(),
                                span: *span,
                            })
                        }
                    }
                    BinaryOperator::Multiply => {
                        match (&l, &r) {
                            (Value::Number(a), Value::Number(b)) => {
                                a.checked_mul(*b)
                                    .map(Value::Number)
                                    .ok_or(NumerusError::IntegerOverflow {
                                        value: *a as i64 * *b as i64,
                                    })
                            }
                            _ => Err(NumerusError::TypeMismatch {
                                operation: "MULTIPLICA".to_string(),
                                expected: "numbers".to_string(),
                                span: *span,
                            })
                        }
                    }
                    BinaryOperator::Divide => {
                        match (&l, &r) {
                            (Value::Number(a), Value::Number(b)) => {
                                if *b == 0 {
                                    Err(NumerusError::DivisionByZero { span: *span })
                                } else {
                                    Ok(Value::Number(a / b))
                                }
                            }
                            _ => Err(NumerusError::TypeMismatch {
                                operation: "DIVIDE".to_string(),
                                expected: "numbers".to_string(),
                                span: *span,
                            })
                        }
                    }
                }
            }

            Expression::Grouped { inner, .. } => self.evaluate_expression(inner),

            Expression::FunctionCall { function, argument, span } => {
                let arg_value = self.evaluate_expression(argument)?;

                match function {
                    BuiltinFunction::Romaniza => {
                        // ROMANIZA converts a number to its Roman string representation
                        match arg_value {
                            Value::Number(n) => {
                                let roman = to_roman(n).map_err(|_| {
                                    NumerusError::RomanOverflow { value: n }
                                })?;
                                Ok(Value::String(roman))
                            }
                            Value::String(_) => Err(NumerusError::TypeMismatch {
                                operation: "ROMANIZA".to_string(),
                                expected: "number".to_string(),
                                span: *span,
                            })
                        }
                    }
                    BuiltinFunction::Arabiza => {
                        // ARABIZA converts a number to its Arabic string representation
                        // This allows displaying numbers as Arabic when concatenating or printing
                        match arg_value {
                            Value::Number(n) => {
                                Ok(Value::String(n.to_string()))
                            }
                            Value::String(_) => Err(NumerusError::TypeMismatch {
                                operation: "ARABIZA".to_string(),
                                expected: "number".to_string(),
                                span: *span,
                            })
                        }
                    }
                    BuiltinFunction::Exprime => {
                        // EXPRIME returns value as-is for now
                        Ok(arg_value)
                    }
                }
            }
        }
    }

    /// Get the environment (for testing/debugging)
    pub fn environment(&self) -> &Environment {
        &self.env
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn run(input: &str) -> Vec<String> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.run(&program).unwrap()
    }

    fn run_and_get_env(input: &str) -> (Vec<String>, Environment) {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        let output = interpreter.run(&program).unwrap();
        (output, interpreter.env.clone())
    }

    #[test]
    fn test_declaration() {
        let (_, env) = run_and_get_env("DECLARA X EST 42");
        assert_eq!(env.get("X").unwrap(), Value::Number(42));
    }

    #[test]
    fn test_roman_declaration() {
        let (_, env) = run_and_get_env("DECLARA X EST XIV");
        assert_eq!(env.get("X").unwrap(), Value::Number(14));
    }

    #[test]
    fn test_string_declaration() {
        let (_, env) = run_and_get_env(r#"DECLARA msg EST "Hello World""#);
        assert_eq!(env.get("msg").unwrap(), Value::String("Hello World".to_string()));
    }

    #[test]
    fn test_lowercase_variable() {
        let (_, env) = run_and_get_env("DECLARA myVar EST 42");
        assert_eq!(env.get("myVar").unwrap(), Value::Number(42));
    }

    #[test]
    fn test_assignment() {
        let (_, env) = run_and_get_env("DECLARA X EST 10\nX EST 42");
        assert_eq!(env.get("X").unwrap(), Value::Number(42));
    }

    #[test]
    fn test_addition() {
        let (_, env) = run_and_get_env("DECLARA A EST 10\nDECLARA B EST 5\nDECLARA C EST A ADDIUS B");
        assert_eq!(env.get("C").unwrap(), Value::Number(15));
    }

    #[test]
    fn test_string_concat() {
        let (_, env) = run_and_get_env(r#"DECLARA msg EST "Hello " ADDIUS "World""#);
        assert_eq!(env.get("msg").unwrap(), Value::String("Hello World".to_string()));
    }

    #[test]
    fn test_string_number_concat() {
        let (_, env) = run_and_get_env(r#"DECLARA msg EST "Value: " ADDIUS 42"#);
        assert_eq!(env.get("msg").unwrap(), Value::String("Value: XLII".to_string()));
    }

    #[test]
    fn test_subtraction() {
        let (_, env) = run_and_get_env("DECLARA A EST 10\nDECLARA B EST 3\nDECLARA C EST A SUBTRAHE B");
        assert_eq!(env.get("C").unwrap(), Value::Number(7));
    }

    #[test]
    fn test_multiplication() {
        let (_, env) = run_and_get_env("DECLARA A EST 6\nDECLARA B EST 7\nDECLARA C EST A MULTIPLICA B");
        assert_eq!(env.get("C").unwrap(), Value::Number(42));
    }

    #[test]
    fn test_division() {
        let (_, env) = run_and_get_env("DECLARA A EST 42\nDECLARA B EST 6\nDECLARA C EST A DIVIDE B");
        assert_eq!(env.get("C").unwrap(), Value::Number(7));
    }

    #[test]
    fn test_precedence() {
        // 2 + 3 * 4 = 2 + 12 = 14
        let (_, env) = run_and_get_env("DECLARA X EST 2 ADDIUS 3 MULTIPLICA 4");
        assert_eq!(env.get("X").unwrap(), Value::Number(14));
    }

    #[test]
    fn test_parentheses() {
        // (2 + 3) * 4 = 5 * 4 = 20
        let (_, env) = run_and_get_env("DECLARA X EST (2 ADDIUS 3) MULTIPLICA 4");
        assert_eq!(env.get("X").unwrap(), Value::Number(20));
    }

    #[test]
    fn test_print_number_roman() {
        let output = run(r#"DECLARA X EST 42
SCRIBE(X)"#);
        assert_eq!(output[0], "XLII");
    }

    #[test]
    fn test_print_number_arabic() {
        let output = run(r#"DECLARA X EST 42
SCRIBE(ARABIZA(X))"#);
        assert_eq!(output[0], "42");
    }

    #[test]
    fn test_arabiza() {
        let (_, env) = run_and_get_env("DECLARA X EST ARABIZA(42)");
        assert_eq!(env.get("X").unwrap(), Value::String("42".to_string()));
    }

    #[test]
    fn test_print_string() {
        let output = run(r#"SCRIBE("Hello World")"#);
        assert_eq!(output[0], "Hello World");
    }

    #[test]
    fn test_print_concat() {
        let output = run(r#"DECLARA X EST 42
SCRIBE("Value: " ADDIUS X)"#);
        assert_eq!(output[0], "Value: XLII");
    }

    #[test]
    fn test_romaniza() {
        let (_, env) = run_and_get_env("DECLARA X EST ROMANIZA(42)");
        assert_eq!(env.get("X").unwrap(), Value::String("XLII".to_string()));
    }

    #[test]
    fn test_complex_expression() {
        // Use multi-char Roman numerals (single chars are identifiers)
        let (_, env) = run_and_get_env(r#"
DECLARA A EST XV
DECLARA B EST 10
DECLARA C EST A ADDIUS B
DECLARA D EST C DIVIDE 5
"#);
        assert_eq!(env.get("A").unwrap(), Value::Number(15));
        assert_eq!(env.get("B").unwrap(), Value::Number(10));
        assert_eq!(env.get("C").unwrap(), Value::Number(25));
        assert_eq!(env.get("D").unwrap(), Value::Number(5));
    }

    #[test]
    fn test_division_by_zero() {
        let mut lexer = Lexer::new("DECLARA X EST 10 DIVIDE 0");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(&program);
        assert!(matches!(result, Err(NumerusError::DivisionByZero { .. })));
    }

    #[test]
    fn test_undefined_variable() {
        let mut lexer = Lexer::new("DECLARA X EST Y");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(&program);
        assert!(matches!(result, Err(NumerusError::UndefinedVariable { .. })));
    }
}
