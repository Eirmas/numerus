use super::{Span, Token, TokenKind};
use crate::error::NumerusError;
use crate::roman::{from_roman, looks_like_roman};

pub struct Lexer<'a> {
    #[allow(dead_code)]
    input: &'a str,
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    current_pos: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.char_indices().peekable(),
            current_pos: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenize the entire input
    pub fn tokenize(&mut self) -> Result<Vec<Token>, NumerusError> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token()? {
            // Skip newlines and comments for now (could keep them for formatting)
            match &token.kind {
                TokenKind::Newline | TokenKind::Comment(_) => continue,
                _ => tokens.push(token),
            }
        }

        tokens.push(Token::new(
            TokenKind::Eof,
            Span::point(self.current_pos, self.line, self.column),
            String::new(),
        ));

        Ok(tokens)
    }

    /// Get the next token
    fn next_token(&mut self) -> Result<Option<Token>, NumerusError> {
        self.skip_whitespace();

        let Some(&(start, ch)) = self.chars.peek() else {
            return Ok(None);
        };

        match ch {
            '(' => self.single_char_token(TokenKind::LeftParen),
            ')' => self.single_char_token(TokenKind::RightParen),
            '{' => self.single_char_token(TokenKind::LeftBrace),
            '}' => self.single_char_token(TokenKind::RightBrace),
            ',' => self.single_char_token(TokenKind::Comma),
            '"' => self.read_string(),
            '\n' => {
                let col = self.column;
                self.advance();
                self.line += 1;
                self.column = 1;
                Ok(Some(Token::new(
                    TokenKind::Newline,
                    Span::new(start, start + 1, self.line - 1, col),
                    "\n".to_string(),
                )))
            }
            'A'..='Z' | 'a'..='z' | '_' => self.read_identifier_or_keyword(),
            '0'..='9' => self.read_arabic_number(),
            _ => Err(NumerusError::UnexpectedCharacter {
                ch,
                line: self.line,
                column: self.column,
            }),
        }
    }

    /// Advance to the next character
    fn advance(&mut self) -> Option<(usize, char)> {
        if let Some((pos, ch)) = self.chars.next() {
            self.current_pos = pos + ch.len_utf8();
            self.column += 1;
            Some((pos, ch))
        } else {
            None
        }
    }

    /// Skip whitespace (except newlines)
    fn skip_whitespace(&mut self) {
        while let Some(&(_, ch)) = self.chars.peek() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Create a single-character token
    fn single_char_token(&mut self, kind: TokenKind) -> Result<Option<Token>, NumerusError> {
        let start = self.current_pos;
        let col = self.column;
        let (_, ch) = self.advance().unwrap();
        Ok(Some(Token::new(
            kind,
            Span::new(start, self.current_pos, self.line, col),
            ch.to_string(),
        )))
    }

    /// Read an identifier or keyword
    fn read_identifier_or_keyword(&mut self) -> Result<Option<Token>, NumerusError> {
        let start = self.current_pos;
        let start_column = self.column;
        let mut lexeme = String::new();

        while let Some(&(_, ch)) = self.chars.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                lexeme.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let span = Span::new(start, self.current_pos, self.line, start_column);

        // Check if it's NOTA: (comment)
        if lexeme == "NOTA" {
            if self.chars.peek().map(|&(_, c)| c) == Some(':') {
                self.advance(); // consume ':'
                return self.read_comment(start, start_column);
            }
        }

        // Check for keywords
        let kind = match lexeme.as_str() {
            "DECLARA" => TokenKind::Declara,
            "EST" => TokenKind::Est,
            "ADDIUS" => TokenKind::Addius,
            "SUBTRAHE" => TokenKind::Subtrahe,
            "MULTIPLICA" => TokenKind::Multiplica,
            "DIVIDE" => TokenKind::Divide,
            "SCRIBE" => TokenKind::Scribe,
            "AVTEM" => TokenKind::Avtem,
            "ROMANIZA" => TokenKind::Romaniza,
            "ARABIZA" => TokenKind::Arabiza,
            "EXPRIME" => TokenKind::Exprime,
            _ => {
                // Check if it's a valid Roman numeral
                // Only treat as Roman numeral if:
                // 1. It looks like a Roman numeral (only I, V, X, L, C, D, M)
                // 2. It's at least 2 characters (single chars are identifiers)
                // 3. It parses successfully
                if lexeme.len() >= 2 && looks_like_roman(&lexeme) {
                    match from_roman(&lexeme) {
                        Ok(value) => TokenKind::RomanLiteral(value),
                        Err(_) => {
                            // Not a valid Roman numeral, treat as identifier
                            TokenKind::Identifier(lexeme.clone())
                        }
                    }
                } else {
                    TokenKind::Identifier(lexeme.clone())
                }
            }
        };

        Ok(Some(Token::new(kind, span, lexeme)))
    }

    /// Read a comment (after NOTA:)
    fn read_comment(&mut self, start: usize, start_column: usize) -> Result<Option<Token>, NumerusError> {
        let mut comment = String::new();

        // Read until end of line
        while let Some(&(_, ch)) = self.chars.peek() {
            if ch == '\n' {
                break;
            }
            comment.push(ch);
            self.advance();
        }

        Ok(Some(Token::new(
            TokenKind::Comment(comment.trim().to_string()),
            Span::new(start, self.current_pos, self.line, start_column),
            format!("NOTA: {}", comment),
        )))
    }

    /// Read an Arabic number literal
    fn read_arabic_number(&mut self) -> Result<Option<Token>, NumerusError> {
        let start = self.current_pos;
        let start_column = self.column;
        let mut lexeme = String::new();

        while let Some(&(_, ch)) = self.chars.peek() {
            if ch.is_ascii_digit() {
                lexeme.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let span = Span::new(start, self.current_pos, self.line, start_column);

        let value: i64 = lexeme.parse().unwrap_or(0);
        if value > 3999 {
            return Err(NumerusError::NumberOutOfRange { value, span });
        }

        Ok(Some(Token::new(
            TokenKind::ArabicLiteral(value as i32),
            span,
            lexeme,
        )))
    }

    /// Read a string literal with template placeholders
    fn read_string(&mut self) -> Result<Option<Token>, NumerusError> {
        let start = self.current_pos;
        let start_column = self.column;
        let start_line = self.line;

        self.advance(); // consume opening quote

        let mut content = String::new();

        loop {
            match self.chars.peek() {
                Some(&(_, '"')) => {
                    self.advance(); // consume closing quote
                    break;
                }
                Some(&(_, '\n')) | None => {
                    return Err(NumerusError::UnterminatedString { line: start_line });
                }
                Some(&(_, ch)) => {
                    content.push(ch);
                    self.advance();
                }
            }
        }

        Ok(Some(Token::new(
            TokenKind::StringLiteral(content.clone()),
            Span::new(start, self.current_pos, self.line, start_column),
            format!("\"{}\"", content),
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(input: &str) -> Vec<TokenKind> {
        let mut lexer = Lexer::new(input);
        lexer.tokenize().unwrap().into_iter().map(|t| t.kind).collect()
    }

    #[test]
    fn test_keywords() {
        let tokens = tokenize("DECLARA EST ADDIUS SUBTRAHE MULTIPLICA DIVIDE SCRIBE ARABIZA AVTEM");
        assert_eq!(tokens, vec![
            TokenKind::Declara,
            TokenKind::Est,
            TokenKind::Addius,
            TokenKind::Subtrahe,
            TokenKind::Multiplica,
            TokenKind::Divide,
            TokenKind::Scribe,
            TokenKind::Arabiza,
            TokenKind::Avtem,
            TokenKind::Eof,
        ]);
    }

    #[test]
    fn test_roman_literals() {
        // Single chars are identifiers, multi-char Roman numerals are literals
        let tokens = tokenize("II IV IX XIV XLII MCMXCIX");
        assert_eq!(tokens, vec![
            TokenKind::RomanLiteral(2),
            TokenKind::RomanLiteral(4),
            TokenKind::RomanLiteral(9),
            TokenKind::RomanLiteral(14),
            TokenKind::RomanLiteral(42),
            TokenKind::RomanLiteral(1999),
            TokenKind::Eof,
        ]);
    }

    #[test]
    fn test_single_roman_chars_are_identifiers() {
        // Single Roman numeral characters should be identifiers (for variable names)
        let tokens = tokenize("I V X L C D M");
        assert_eq!(tokens, vec![
            TokenKind::Identifier("I".to_string()),
            TokenKind::Identifier("V".to_string()),
            TokenKind::Identifier("X".to_string()),
            TokenKind::Identifier("L".to_string()),
            TokenKind::Identifier("C".to_string()),
            TokenKind::Identifier("D".to_string()),
            TokenKind::Identifier("M".to_string()),
            TokenKind::Eof,
        ]);
    }

    #[test]
    fn test_arabic_literals() {
        let tokens = tokenize("0 1 42 3999");
        assert_eq!(tokens, vec![
            TokenKind::ArabicLiteral(0),
            TokenKind::ArabicLiteral(1),
            TokenKind::ArabicLiteral(42),
            TokenKind::ArabicLiteral(3999),
            TokenKind::Eof,
        ]);
    }

    #[test]
    fn test_identifiers() {
        let tokens = tokenize("VARIABILIS NUMERUS RES");
        assert_eq!(tokens, vec![
            TokenKind::Identifier("VARIABILIS".to_string()),
            TokenKind::Identifier("NUMERUS".to_string()),
            TokenKind::Identifier("RES".to_string()),
            TokenKind::Eof,
        ]);
    }

    #[test]
    fn test_string_literal() {
        let tokens = tokenize(r#""SALVE MUNDE""#);
        assert_eq!(tokens, vec![
            TokenKind::StringLiteral("SALVE MUNDE".to_string()),
            TokenKind::Eof,
        ]);
    }

    #[test]
    fn test_string_with_placeholder() {
        let tokens = tokenize(r#""VALOR: {X}""#);
        assert_eq!(tokens, vec![
            TokenKind::StringLiteral("VALOR: {X}".to_string()),
            TokenKind::Eof,
        ]);
    }

    #[test]
    fn test_punctuation() {
        let tokens = tokenize("( ) { } ,");
        assert_eq!(tokens, vec![
            TokenKind::LeftParen,
            TokenKind::RightParen,
            TokenKind::LeftBrace,
            TokenKind::RightBrace,
            TokenKind::Comma,
            TokenKind::Eof,
        ]);
    }

    #[test]
    fn test_declaration() {
        let tokens = tokenize("DECLARA X EST 42");
        assert_eq!(tokens, vec![
            TokenKind::Declara,
            TokenKind::Identifier("X".to_string()),
            TokenKind::Est,
            TokenKind::ArabicLiteral(42),
            TokenKind::Eof,
        ]);
    }

    #[test]
    fn test_expression() {
        let tokens = tokenize("A ADDIUS B MULTIPLICA C");
        assert_eq!(tokens, vec![
            TokenKind::Identifier("A".to_string()),
            TokenKind::Addius,
            TokenKind::Identifier("B".to_string()),
            TokenKind::Multiplica,
            TokenKind::Identifier("C".to_string()),
            TokenKind::Eof,
        ]);
    }

    #[test]
    fn test_number_out_of_range() {
        let mut lexer = Lexer::new("4000");
        let result = lexer.tokenize();
        assert!(result.is_err());
    }

    #[test]
    fn test_unterminated_string() {
        let mut lexer = Lexer::new("\"hello");
        let result = lexer.tokenize();
        assert!(result.is_err());
    }
}
