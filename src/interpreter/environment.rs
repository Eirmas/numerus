use std::collections::HashMap;
use crate::error::NumerusError;
use crate::roman::to_roman;

/// Runtime value - can be a number or a string
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i32),
    String(String),
}

impl Value {
    /// Convert value to string for output (numbers are displayed as Roman numerals)
    pub fn to_output_string(&self) -> Result<String, NumerusError> {
        match self {
            Value::String(s) => Ok(s.clone()),
            Value::Number(n) => {
                // Convert to Roman
                if *n <= 0 {
                    return Err(NumerusError::NegativeRomanConversion { value: *n });
                }
                if *n > 3999 {
                    return Err(NumerusError::RomanOverflow { value: *n });
                }
                to_roman(*n).map_err(|_| NumerusError::RomanOverflow { value: *n })
            }
        }
    }

    /// Check if this is a number
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    /// Check if this is a string
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    /// Get as number, if it is one
    pub fn as_number(&self) -> Option<i32> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Get as string, if it is one
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}

/// Symbol table for variable storage
#[derive(Debug, Clone, Default)]
pub struct Environment {
    variables: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Declare a new variable
    pub fn declare(&mut self, name: String, value: Value) -> Result<(), NumerusError> {
        if self.variables.contains_key(&name) {
            return Err(NumerusError::VariableAlreadyDeclared { name });
        }
        self.variables.insert(name, value);
        Ok(())
    }

    /// Assign to an existing variable
    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), NumerusError> {
        if !self.variables.contains_key(name) {
            return Err(NumerusError::UndefinedVariable {
                name: name.to_string(),
            });
        }
        self.variables.insert(name.to_string(), value);
        Ok(())
    }

    /// Get a variable's value
    pub fn get(&self, name: &str) -> Result<Value, NumerusError> {
        self.variables.get(name).cloned().ok_or_else(|| {
            NumerusError::UndefinedVariable {
                name: name.to_string(),
            }
        })
    }

    /// Check if a variable exists
    pub fn contains(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    /// Get all variable names (for debugging/REPL)
    pub fn variables(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.variables.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_declare_and_get_number() {
        let mut env = Environment::new();
        env.declare("X".to_string(), Value::Number(42)).unwrap();
        assert_eq!(env.get("X").unwrap(), Value::Number(42));
    }

    #[test]
    fn test_declare_and_get_string() {
        let mut env = Environment::new();
        env.declare("msg".to_string(), Value::String("Hello".to_string())).unwrap();
        assert_eq!(env.get("msg").unwrap(), Value::String("Hello".to_string()));
    }

    #[test]
    fn test_declare_twice_fails() {
        let mut env = Environment::new();
        env.declare("X".to_string(), Value::Number(42)).unwrap();
        let result = env.declare("X".to_string(), Value::Number(100));
        assert!(result.is_err());
    }

    #[test]
    fn test_assign_existing() {
        let mut env = Environment::new();
        env.declare("X".to_string(), Value::Number(42)).unwrap();
        env.assign("X", Value::Number(100)).unwrap();
        assert_eq!(env.get("X").unwrap(), Value::Number(100));
    }

    #[test]
    fn test_assign_undefined_fails() {
        let mut env = Environment::new();
        let result = env.assign("X", Value::Number(100));
        assert!(result.is_err());
    }

    #[test]
    fn test_get_undefined_fails() {
        let env = Environment::new();
        let result = env.get("X");
        assert!(result.is_err());
    }

    #[test]
    fn test_value_to_output_string() {
        assert_eq!(Value::String("Hello".to_string()).to_output_string().unwrap(), "Hello");
        assert_eq!(Value::Number(42).to_output_string().unwrap(), "XLII");
    }
}
