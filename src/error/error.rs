use thiserror::Error;
use crate::lexer::Span;

/// All error types for Numerus++ with Latin-style messages
#[derive(Error, Debug, Clone, PartialEq)]
pub enum NumerusError {
    // ═══════════════════════════════════════════════════════════
    // LEXER ERRORS (ERRATA LEXICA)
    // ═══════════════════════════════════════════════════════════

    #[error("ERRATUM LEXICUM: Character '{ch}' ignotum est in linea {line}, columna {column}!")]
    UnexpectedCharacter {
        ch: char,
        line: usize,
        column: usize,
    },

    #[error("ERRATUM LEXICUM: Numerus Romanus '{numeral}' invalidus est!")]
    InvalidRomanNumeral {
        numeral: String,
        span: Span,
    },

    #[error("ERRATUM LEXICUM: String non terminata in linea {line}!")]
    UnterminatedString {
        line: usize,
    },

    #[error("ERRATUM LEXICUM: Numerus {value} extra fines est! (I-MMMCMXCIX solum)")]
    NumberOutOfRange {
        value: i64,
        span: Span,
    },

    // ═══════════════════════════════════════════════════════════
    // PARSER ERRORS (ERRATA SYNTAXIS)
    // ═══════════════════════════════════════════════════════════

    #[error("ERRATUM SYNTAXIS: Expectabatur '{expected}', sed inveni '{found}'!")]
    UnexpectedToken {
        expected: String,
        found: String,
        span: Span,
    },

    #[error("ERRATUM SYNTAXIS: Expressio expectata post '{after}'!")]
    ExpectedExpression {
        after: String,
        span: Span,
    },

    #[error("ERRATUM SYNTAXIS: Parenthesis clausa ')' desideratur!")]
    UnclosedParenthesis {
        opening_span: Span,
    },

    #[error("ERRATUM SYNTAXIS: Finis inexpectatus programmatis!")]
    UnexpectedEndOfInput,

    #[error("ERRATUM SYNTAXIS: Identificator expectatus!")]
    ExpectedIdentifier {
        span: Span,
    },

    // ═══════════════════════════════════════════════════════════
    // RUNTIME ERRORS (ERRATA TEMPORIS EXECUTIONIS)
    // ═══════════════════════════════════════════════════════════

    #[error("ERRATUM: Variable '{name}' non declarata est!")]
    UndefinedVariable {
        name: String,
    },

    #[error("ERRATUM: Variable '{name}' iam declarata est!")]
    VariableAlreadyDeclared {
        name: String,
    },

    #[error("ERRATUM: Divisio per nihilum prohibita est! (Etiam Romani hoc sciebant)")]
    DivisionByZero {
        span: Span,
    },

    #[error("ERRATUM: Numerus negativus {value} in Romanis exprimi non potest!")]
    NegativeRomanConversion {
        value: i32,
    },

    #[error("ERRATUM: Numerus {value} nimis magnus pro Romanis (maximum MMMCMXCIX)!")]
    RomanOverflow {
        value: i32,
    },

    #[error("ERRATUM: Numerus {value} nimis magnus vel parvus!")]
    IntegerOverflow {
        value: i64,
    },

    #[error("ERRATUM: Operatio '{operation}' requirit {expected}!")]
    TypeMismatch {
        operation: String,
        expected: String,
        span: Span,
    },

    // ═══════════════════════════════════════════════════════════
    // BUILT-IN FUNCTION ERRORS
    // ═══════════════════════════════════════════════════════════

    #[error("ERRATUM: Functio '{name}' argumentum invalidum accepit!")]
    InvalidFunctionArgument {
        name: String,
        span: Span,
    },
}

impl NumerusError {
    /// Get the span where this error occurred, if available
    pub fn span(&self) -> Option<Span> {
        match self {
            Self::InvalidRomanNumeral { span, .. } => Some(*span),
            Self::NumberOutOfRange { span, .. } => Some(*span),
            Self::UnexpectedToken { span, .. } => Some(*span),
            Self::ExpectedExpression { span, .. } => Some(*span),
            Self::UnclosedParenthesis { opening_span } => Some(*opening_span),
            Self::ExpectedIdentifier { span } => Some(*span),
            Self::DivisionByZero { span } => Some(*span),
            Self::TypeMismatch { span, .. } => Some(*span),
            Self::InvalidFunctionArgument { span, .. } => Some(*span),
            _ => None,
        }
    }
}

/// Format an error with source context for pretty printing
pub fn format_error_with_context(source: &str, error: &NumerusError) -> String {
    let mut output = format!("{}\n", error);

    if let Some(span) = error.span() {
        if let Some(line) = source.lines().nth(span.line.saturating_sub(1)) {
            output.push_str(&format!(
                "  --> linea {}:{}\n   |\n {:>3} | {}\n   | {}{}\n",
                span.line,
                span.column,
                span.line,
                line,
                " ".repeat(span.column.saturating_sub(1)),
                "^".repeat((span.end - span.start).max(1))
            ));
        }
    }

    output
}
