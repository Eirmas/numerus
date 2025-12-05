use crate::lexer::Span;

/// The root of the AST - a program is a list of statements
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

/// All statement types in Numerus++
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// DECLARA <IDENT> EST <VALUE>
    Declaration {
        name: String,
        value: Expression,
        span: Span,
    },

    /// <IDENT> EST <EXPR>
    Assignment {
        name: String,
        value: Expression,
        span: Span,
    },

    /// SCRIBE(expression)
    /// Expression can be a string, number, or concatenation
    Print {
        value: Expression,
        span: Span,
    },

    /// AVTEM - ceremonial no-op
    Avtem {
        span: Span,
    },

    /// NOTA: ... - comment (preserved in AST for tooling)
    Comment {
        text: String,
        span: Span,
    },
}

/// Expression AST node
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// A numeric literal (Arabic or Roman, stored as i32)
    NumberLiteral {
        value: i32,
        original_form: NumberForm,
        span: Span,
    },

    /// A string literal
    StringLiteral {
        value: String,
        span: Span,
    },

    /// Variable reference
    Variable {
        name: String,
        span: Span,
    },

    /// Binary operation (works for both numbers and strings with ADDIUS)
    BinaryOp {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
        span: Span,
    },

    /// Parenthesized expression
    Grouped {
        inner: Box<Expression>,
        span: Span,
    },

    /// Built-in function call: ROMANIZA(n) or EXPRIME(s)
    FunctionCall {
        function: BuiltinFunction,
        argument: Box<Expression>,
        span: Span,
    },
}

impl Expression {
    /// Get the span of this expression
    pub fn span(&self) -> Span {
        match self {
            Expression::NumberLiteral { span, .. } => *span,
            Expression::StringLiteral { span, .. } => *span,
            Expression::Variable { span, .. } => *span,
            Expression::BinaryOp { span, .. } => *span,
            Expression::Grouped { span, .. } => *span,
            Expression::FunctionCall { span, .. } => *span,
        }
    }
}

/// Tracks whether a literal was written as Roman or Arabic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberForm {
    Arabic,
    Roman,
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,        // ADDIUS
    Subtract,   // SUBTRAHE
    Multiply,   // MULTIPLICA
    Divide,     // DIVIDE
}

impl BinaryOperator {
    pub fn symbol(&self) -> &'static str {
        match self {
            BinaryOperator::Add => "ADDIUS",
            BinaryOperator::Subtract => "SUBTRAHE",
            BinaryOperator::Multiply => "MULTIPLICA",
            BinaryOperator::Divide => "DIVIDE",
        }
    }
}

/// Built-in functions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinFunction {
    Romaniza,   // Convert number to Roman numeral string
    Arabiza,    // Convert to Arabic number (for display as decimal)
    Exprime,    // Convert Roman string to Arabic (for future string support)
}

