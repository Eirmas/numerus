use super::Span;

/// Token with its kind and location
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub lexeme: String,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, lexeme: String) -> Self {
        Self { kind, span, lexeme }
    }
}

/// All possible token types in Numerus++
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // ═══════════════════════════════════════════════════════════
    // KEYWORDS (VERBA CLAVIS)
    // ═══════════════════════════════════════════════════════════
    Declara,        // DECLARA - declare variable
    Est,            // EST - assignment/equals
    Addius,         // ADDIUS - addition
    Subtrahe,       // SUBTRAHE - subtraction
    Multiplica,     // MULTIPLICA - multiplication
    Divide,         // DIVIDE - division
    Scribe,         // SCRIBE - print
    Avtem,          // AVTEM - ceremonial no-op

    // ═══════════════════════════════════════════════════════════
    // BUILT-IN FUNCTIONS (FUNCTIONES)
    // ═══════════════════════════════════════════════════════════
    Romaniza,       // ROMANIZA - convert number to Roman string
    Arabiza,        // ARABIZA - convert to Arabic number
    Exprime,        // EXPRIME - expression evaluation

    // ═══════════════════════════════════════════════════════════
    // LITERALS (LITTERAE)
    // ═══════════════════════════════════════════════════════════
    ArabicLiteral(i32),     // 0-3999
    RomanLiteral(i32),      // Stored as Arabic internally
    StringLiteral(String),  // "...{VAR}..."

    // ═══════════════════════════════════════════════════════════
    // IDENTIFIERS
    // ═══════════════════════════════════════════════════════════
    Identifier(String),

    // ═══════════════════════════════════════════════════════════
    // PUNCTUATION
    // ═══════════════════════════════════════════════════════════
    LeftParen,      // (
    RightParen,     // )
    LeftBrace,      // {
    RightBrace,     // }
    Comma,          // ,

    // ═══════════════════════════════════════════════════════════
    // SPECIAL
    // ═══════════════════════════════════════════════════════════
    Comment(String), // NOTA: ...
    Newline,
    Eof,
}

impl TokenKind {
    /// Returns true if this token is a binary operator
    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            TokenKind::Addius | TokenKind::Subtrahe |
            TokenKind::Multiplica | TokenKind::Divide
        )
    }

    /// Returns true if this is an additive operator (lower precedence)
    pub fn is_additive(&self) -> bool {
        matches!(self, TokenKind::Addius | TokenKind::Subtrahe)
    }

    /// Returns true if this is a multiplicative operator (higher precedence)
    pub fn is_multiplicative(&self) -> bool {
        matches!(self, TokenKind::Multiplica | TokenKind::Divide)
    }

    /// Get human-readable name for error messages
    pub fn name(&self) -> &'static str {
        match self {
            TokenKind::Declara => "DECLARA",
            TokenKind::Est => "EST",
            TokenKind::Addius => "ADDIUS",
            TokenKind::Subtrahe => "SUBTRAHE",
            TokenKind::Multiplica => "MULTIPLICA",
            TokenKind::Divide => "DIVIDE",
            TokenKind::Scribe => "SCRIBE",
            TokenKind::Avtem => "AVTEM",
            TokenKind::Romaniza => "ROMANIZA",
            TokenKind::Arabiza => "ARABIZA",
            TokenKind::Exprime => "EXPRIME",
            TokenKind::ArabicLiteral(_) => "numerus Arabicus",
            TokenKind::RomanLiteral(_) => "numerus Romanus",
            TokenKind::StringLiteral(_) => "string",
            TokenKind::Identifier(_) => "identificator",
            TokenKind::LeftParen => "(",
            TokenKind::RightParen => ")",
            TokenKind::LeftBrace => "{",
            TokenKind::RightBrace => "}",
            TokenKind::Comma => ",",
            TokenKind::Comment(_) => "NOTA",
            TokenKind::Newline => "linea nova",
            TokenKind::Eof => "finis",
        }
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
