use super::ast::*;
use crate::error::NumerusError;
use crate::lexer::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Parse the entire program
    pub fn parse(&mut self) -> Result<Program, NumerusError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        Ok(Program::new(statements))
    }

    /// Parse a single statement
    fn parse_statement(&mut self) -> Result<Statement, NumerusError> {
        match &self.peek().kind {
            TokenKind::Declara => self.parse_declaration(),
            TokenKind::Scribe => self.parse_print(),
            TokenKind::Avtem => self.parse_avtem(),
            TokenKind::Comment(text) => {
                let text = text.clone();
                let token = self.advance();
                Ok(Statement::Comment {
                    text,
                    span: token.span,
                })
            }
            TokenKind::Identifier(_) => self.parse_assignment(),
            TokenKind::Eof => Err(NumerusError::UnexpectedEndOfInput),
            _ => Err(NumerusError::UnexpectedToken {
                expected: "DECLARA, SCRIBE, AVTEM, or identifier".to_string(),
                found: format!("{}", self.peek().kind),
                span: self.peek().span,
            }),
        }
    }

    /// Parse: DECLARA <IDENT> EST <EXPR>
    fn parse_declaration(&mut self) -> Result<Statement, NumerusError> {
        let start_span = self.advance().span; // consume DECLARA

        let name = self.expect_identifier()?;
        self.expect_token(TokenKind::Est)?;
        let value = self.parse_expression()?;

        Ok(Statement::Declaration {
            name,
            value: value.clone(),
            span: start_span.merge(&value.span()),
        })
    }

    /// Parse: <IDENT> EST <EXPR>
    fn parse_assignment(&mut self) -> Result<Statement, NumerusError> {
        let name_token = self.advance();
        let name = match &name_token.kind {
            TokenKind::Identifier(n) => n.clone(),
            _ => return Err(NumerusError::ExpectedIdentifier { span: name_token.span }),
        };

        self.expect_token(TokenKind::Est)?;
        let value = self.parse_expression()?;

        Ok(Statement::Assignment {
            name,
            value: value.clone(),
            span: name_token.span.merge(&value.span()),
        })
    }

    /// Parse: SCRIBE(expression)
    fn parse_print(&mut self) -> Result<Statement, NumerusError> {
        let start_span = self.advance().span; // consume SCRIBE
        self.expect_token(TokenKind::LeftParen)?;

        let value = self.parse_expression()?;

        let end_token = self.expect_token(TokenKind::RightParen)?;

        Ok(Statement::Print {
            value,
            span: start_span.merge(&end_token.span),
        })
    }

    /// Parse: AVTEM
    fn parse_avtem(&mut self) -> Result<Statement, NumerusError> {
        let token = self.advance();
        Ok(Statement::Avtem { span: token.span })
    }

    /// Parse an expression with proper operator precedence
    /// expression ::= additive
    fn parse_expression(&mut self) -> Result<Expression, NumerusError> {
        self.parse_additive()
    }

    /// Parse additive expressions (lowest precedence)
    /// additive ::= multiplicative ((ADDIUS|SUBTRAHE) multiplicative)*
    fn parse_additive(&mut self) -> Result<Expression, NumerusError> {
        let mut left = self.parse_multiplicative()?;

        while self.peek().kind.is_additive() {
            let op_token = self.advance();
            let operator = match op_token.kind {
                TokenKind::Addius => BinaryOperator::Add,
                TokenKind::Subtrahe => BinaryOperator::Subtract,
                _ => unreachable!(),
            };

            let right = self.parse_multiplicative()?;
            let span = left.span().merge(&right.span());

            left = Expression::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                span,
            };
        }

        Ok(left)
    }

    /// Parse multiplicative expressions (higher precedence)
    /// multiplicative ::= factor ((MULTIPLICA|DIVIDE) factor)*
    fn parse_multiplicative(&mut self) -> Result<Expression, NumerusError> {
        let mut left = self.parse_factor()?;

        while self.peek().kind.is_multiplicative() {
            let op_token = self.advance();
            let operator = match op_token.kind {
                TokenKind::Multiplica => BinaryOperator::Multiply,
                TokenKind::Divide => BinaryOperator::Divide,
                _ => unreachable!(),
            };

            let right = self.parse_factor()?;
            let span = left.span().merge(&right.span());

            left = Expression::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                span,
            };
        }

        Ok(left)
    }

    /// Parse a factor (highest precedence)
    /// factor ::= number | string | identifier | "(" expression ")" | function_call
    fn parse_factor(&mut self) -> Result<Expression, NumerusError> {
        let token = self.peek().clone();

        match &token.kind {
            TokenKind::ArabicLiteral(n) => {
                let value = *n;
                self.advance();
                Ok(Expression::NumberLiteral {
                    value,
                    original_form: NumberForm::Arabic,
                    span: token.span,
                })
            }
            TokenKind::RomanLiteral(n) => {
                let value = *n;
                self.advance();
                Ok(Expression::NumberLiteral {
                    value,
                    original_form: NumberForm::Roman,
                    span: token.span,
                })
            }
            TokenKind::StringLiteral(s) => {
                let value = s.clone();
                self.advance();
                Ok(Expression::StringLiteral {
                    value,
                    span: token.span,
                })
            }
            TokenKind::Identifier(_) => {
                self.advance();
                if let TokenKind::Identifier(name) = token.kind {
                    Ok(Expression::Variable {
                        name,
                        span: token.span,
                    })
                } else {
                    unreachable!()
                }
            }
            TokenKind::LeftParen => {
                let open = self.advance();
                let inner = self.parse_expression()?;
                let close = self.expect_token(TokenKind::RightParen)?;
                Ok(Expression::Grouped {
                    inner: Box::new(inner),
                    span: open.span.merge(&close.span),
                })
            }
            TokenKind::Romaniza => self.parse_function_call(BuiltinFunction::Romaniza),
            TokenKind::Arabiza => self.parse_function_call(BuiltinFunction::Arabiza),
            TokenKind::Exprime => self.parse_function_call(BuiltinFunction::Exprime),
            _ => Err(NumerusError::ExpectedExpression {
                after: if self.current > 0 {
                    format!("{}", self.previous().kind)
                } else {
                    "start".to_string()
                },
                span: token.span,
            }),
        }
    }

    /// Parse a built-in function call: ROMANIZA(expr) or EXPRIME(expr)
    fn parse_function_call(&mut self, function: BuiltinFunction) -> Result<Expression, NumerusError> {
        let start = self.advance().span;
        self.expect_token(TokenKind::LeftParen)?;
        let argument = self.parse_expression()?;
        let end = self.expect_token(TokenKind::RightParen)?;

        Ok(Expression::FunctionCall {
            function,
            argument: Box::new(argument),
            span: start.merge(&end.span),
        })
    }

    // ═══════════════════════════════════════════════════════════
    // Helper methods
    // ═══════════════════════════════════════════════════════════

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current.saturating_sub(1)]
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous().clone()
    }

    fn check(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind)
    }

    fn expect_token(&mut self, expected: TokenKind) -> Result<Token, NumerusError> {
        if std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(&expected) {
            Ok(self.advance())
        } else {
            Err(NumerusError::UnexpectedToken {
                expected: format!("{}", expected),
                found: format!("{}", self.peek().kind),
                span: self.peek().span,
            })
        }
    }

    fn expect_identifier(&mut self) -> Result<String, NumerusError> {
        match &self.peek().kind {
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            _ => Err(NumerusError::ExpectedIdentifier { span: self.peek().span }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(input: &str) -> Program {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse().unwrap()
    }

    #[test]
    fn test_parse_declaration() {
        let program = parse("DECLARA X EST 42");
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Declaration { name, value, .. } => {
                assert_eq!(name, "X");
                match value {
                    Expression::NumberLiteral { value, original_form, .. } => {
                        assert_eq!(*value, 42);
                        assert_eq!(*original_form, NumberForm::Arabic);
                    }
                    _ => panic!("Expected literal"),
                }
            }
            _ => panic!("Expected declaration"),
        }
    }

    #[test]
    fn test_parse_roman_literal() {
        let program = parse("DECLARA X EST XIV");
        match &program.statements[0] {
            Statement::Declaration { value, .. } => {
                match value {
                    Expression::NumberLiteral { value, original_form, .. } => {
                        assert_eq!(*value, 14);
                        assert_eq!(*original_form, NumberForm::Roman);
                    }
                    _ => panic!("Expected literal"),
                }
            }
            _ => panic!("Expected declaration"),
        }
    }

    #[test]
    fn test_parse_string_literal() {
        let program = parse(r#"DECLARA msg EST "Hello World""#);
        match &program.statements[0] {
            Statement::Declaration { name, value, .. } => {
                assert_eq!(name, "msg");
                match value {
                    Expression::StringLiteral { value, .. } => {
                        assert_eq!(value, "Hello World");
                    }
                    _ => panic!("Expected string literal"),
                }
            }
            _ => panic!("Expected declaration"),
        }
    }

    #[test]
    fn test_parse_lowercase_identifier() {
        let program = parse("DECLARA myVar EST 42");
        match &program.statements[0] {
            Statement::Declaration { name, .. } => {
                assert_eq!(name, "myVar");
            }
            _ => panic!("Expected declaration"),
        }
    }

    #[test]
    fn test_parse_assignment() {
        let program = parse("X EST 10");
        match &program.statements[0] {
            Statement::Assignment { name, .. } => {
                assert_eq!(name, "X");
            }
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_binary_expression() {
        let program = parse("X EST A ADDIUS B");
        match &program.statements[0] {
            Statement::Assignment { value, .. } => {
                match value {
                    Expression::BinaryOp { operator, .. } => {
                        assert_eq!(*operator, BinaryOperator::Add);
                    }
                    _ => panic!("Expected binary op"),
                }
            }
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_precedence() {
        // A ADDIUS B MULTIPLICA C should parse as A + (B * C)
        let program = parse("X EST A ADDIUS B MULTIPLICA C");
        match &program.statements[0] {
            Statement::Assignment { value, .. } => {
                match value {
                    Expression::BinaryOp { operator, left, right, .. } => {
                        assert_eq!(*operator, BinaryOperator::Add);
                        // Left should be A (variable)
                        assert!(matches!(**left, Expression::Variable { .. }));
                        // Right should be B * C
                        match &**right {
                            Expression::BinaryOp { operator, .. } => {
                                assert_eq!(*operator, BinaryOperator::Multiply);
                            }
                            _ => panic!("Right should be multiply"),
                        }
                    }
                    _ => panic!("Expected binary op"),
                }
            }
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_parentheses() {
        // (A ADDIUS B) MULTIPLICA C
        let program = parse("X EST (A ADDIUS B) MULTIPLICA C");
        match &program.statements[0] {
            Statement::Assignment { value, .. } => {
                match value {
                    Expression::BinaryOp { operator, left, .. } => {
                        assert_eq!(*operator, BinaryOperator::Multiply);
                        // Left should be grouped (A + B)
                        match &**left {
                            Expression::Grouped { inner, .. } => {
                                match &**inner {
                                    Expression::BinaryOp { operator, .. } => {
                                        assert_eq!(*operator, BinaryOperator::Add);
                                    }
                                    _ => panic!("Inner should be add"),
                                }
                            }
                            _ => panic!("Left should be grouped"),
                        }
                    }
                    _ => panic!("Expected multiply"),
                }
            }
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_print() {
        let program = parse(r#"SCRIBE("Hello World")"#);
        match &program.statements[0] {
            Statement::Print { value, .. } => {
                match value {
                    Expression::StringLiteral { value, .. } => {
                        assert_eq!(value, "Hello World");
                    }
                    _ => panic!("Expected string literal"),
                }
            }
            _ => panic!("Expected print"),
        }
    }

    #[test]
    fn test_parse_arabiza() {
        let program = parse(r#"SCRIBE(ARABIZA(X))"#);
        match &program.statements[0] {
            Statement::Print { value, .. } => {
                match value {
                    Expression::FunctionCall { function, .. } => {
                        assert_eq!(*function, BuiltinFunction::Arabiza);
                    }
                    _ => panic!("Expected function call"),
                }
            }
            _ => panic!("Expected print"),
        }
    }

    #[test]
    fn test_parse_string_concat() {
        let program = parse(r#"DECLARA msg EST "Hello " ADDIUS "World""#);
        match &program.statements[0] {
            Statement::Declaration { value, .. } => {
                match value {
                    Expression::BinaryOp { operator, .. } => {
                        assert_eq!(*operator, BinaryOperator::Add);
                    }
                    _ => panic!("Expected binary op"),
                }
            }
            _ => panic!("Expected declaration"),
        }
    }

    #[test]
    fn test_parse_avtem() {
        let program = parse("AVTEM");
        assert!(matches!(program.statements[0], Statement::Avtem { .. }));
    }
}
