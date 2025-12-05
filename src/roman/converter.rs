/// Roman numeral conversion utilities
/// Handles bidirectional conversion between Arabic integers and Roman numeral strings

const ROMAN_VALUES: [(i32, &str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

/// Convert an Arabic integer (1-3999) to a Roman numeral string
pub fn to_roman(mut n: i32) -> Result<String, RomanError> {
    if n <= 0 {
        return Err(RomanError::NegativeOrZero(n));
    }
    if n > 3999 {
        return Err(RomanError::Overflow(n));
    }

    let mut result = String::new();
    for (value, symbol) in ROMAN_VALUES {
        while n >= value {
            result.push_str(symbol);
            n -= value;
        }
    }
    Ok(result)
}

/// Convert a Roman numeral string to an Arabic integer
/// Validates proper subtractive notation and symbol rules
pub fn from_roman(s: &str) -> Result<i32, RomanError> {
    if s.is_empty() {
        return Err(RomanError::Empty);
    }

    let s = s.to_uppercase();
    let mut total = 0i32;
    let mut prev_value = 0i32;
    let mut consecutive_count = 1;
    let mut prev_char: Option<char> = None;

    for ch in s.chars().rev() {
        let value = match ch {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => return Err(RomanError::InvalidCharacter(ch)),
        };

        // Check for invalid repetition (V, L, D can't repeat; I, X, C, M max 3 times)
        if let Some(prev) = prev_char {
            if prev == ch {
                consecutive_count += 1;
                match ch {
                    'V' | 'L' | 'D' => return Err(RomanError::InvalidRepetition(ch)),
                    'I' | 'X' | 'C' | 'M' if consecutive_count > 3 => {
                        return Err(RomanError::TooManyRepetitions(ch))
                    }
                    _ => {}
                }
            } else {
                consecutive_count = 1;
            }
        }

        // Subtractive notation: if current value < previous value, subtract it
        if value < prev_value {
            // Validate subtractive pairs
            let valid_subtractive = matches!(
                (value, prev_value),
                (1, 5) | (1, 10) |     // IV, IX
                (10, 50) | (10, 100) | // XL, XC
                (100, 500) | (100, 1000) // CD, CM
            );
            if !valid_subtractive {
                return Err(RomanError::InvalidSubtractive(s.clone()));
            }
            total -= value;
        } else {
            total += value;
        }

        prev_value = value;
        prev_char = Some(ch);
    }

    // Final validation: convert back and check it matches
    if let Ok(reconverted) = to_roman(total) {
        if reconverted != s {
            return Err(RomanError::NonCanonical(s, reconverted));
        }
    }

    Ok(total)
}

/// Check if a string looks like it could be a Roman numeral
pub fn looks_like_roman(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| matches!(c, 'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M'))
}

#[derive(Debug, Clone, PartialEq)]
pub enum RomanError {
    NegativeOrZero(i32),
    Overflow(i32),
    Empty,
    InvalidCharacter(char),
    InvalidRepetition(char),
    TooManyRepetitions(char),
    InvalidSubtractive(String),
    NonCanonical(String, String),
}

impl std::fmt::Display for RomanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RomanError::NegativeOrZero(n) => {
                write!(f, "ERRATUM: Numerus {} negativus vel nihil est! Romani non cognoverunt.", n)
            }
            RomanError::Overflow(n) => {
                write!(f, "ERRATUM: Numerus {} nimis magnus pro Romanis (maximum MMMCMXCIX)!", n)
            }
            RomanError::Empty => {
                write!(f, "ERRATUM: Numerus Romanus vacuus est!")
            }
            RomanError::InvalidCharacter(c) => {
                write!(f, "ERRATUM: Character '{}' non est numerus Romanus!", c)
            }
            RomanError::InvalidRepetition(c) => {
                write!(f, "ERRATUM: Littera '{}' non potest repeti!", c)
            }
            RomanError::TooManyRepetitions(c) => {
                write!(f, "ERRATUM: Littera '{}' nimis saepe repetita (maximum III)!", c)
            }
            RomanError::InvalidSubtractive(s) => {
                write!(f, "ERRATUM: Numerus Romanus '{}' subtractivum invalidum habet!", s)
            }
            RomanError::NonCanonical(got, expected) => {
                write!(f, "ERRATUM: '{}' non est forma canonica! Expectabatur '{}'.", got, expected)
            }
        }
    }
}

impl std::error::Error for RomanError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_roman_basic() {
        assert_eq!(to_roman(1).unwrap(), "I");
        assert_eq!(to_roman(2).unwrap(), "II");
        assert_eq!(to_roman(3).unwrap(), "III");
        assert_eq!(to_roman(5).unwrap(), "V");
        assert_eq!(to_roman(10).unwrap(), "X");
        assert_eq!(to_roman(50).unwrap(), "L");
        assert_eq!(to_roman(100).unwrap(), "C");
        assert_eq!(to_roman(500).unwrap(), "D");
        assert_eq!(to_roman(1000).unwrap(), "M");
    }

    #[test]
    fn test_to_roman_subtractive() {
        assert_eq!(to_roman(4).unwrap(), "IV");
        assert_eq!(to_roman(9).unwrap(), "IX");
        assert_eq!(to_roman(40).unwrap(), "XL");
        assert_eq!(to_roman(90).unwrap(), "XC");
        assert_eq!(to_roman(400).unwrap(), "CD");
        assert_eq!(to_roman(900).unwrap(), "CM");
    }

    #[test]
    fn test_to_roman_complex() {
        assert_eq!(to_roman(14).unwrap(), "XIV");
        assert_eq!(to_roman(42).unwrap(), "XLII");
        assert_eq!(to_roman(1999).unwrap(), "MCMXCIX");
        assert_eq!(to_roman(2024).unwrap(), "MMXXIV");
        assert_eq!(to_roman(3999).unwrap(), "MMMCMXCIX");
    }

    #[test]
    fn test_to_roman_boundaries() {
        assert!(to_roman(0).is_err());
        assert!(to_roman(-1).is_err());
        assert!(to_roman(4000).is_err());
    }

    #[test]
    fn test_from_roman_basic() {
        assert_eq!(from_roman("I").unwrap(), 1);
        assert_eq!(from_roman("V").unwrap(), 5);
        assert_eq!(from_roman("X").unwrap(), 10);
        assert_eq!(from_roman("L").unwrap(), 50);
        assert_eq!(from_roman("C").unwrap(), 100);
        assert_eq!(from_roman("D").unwrap(), 500);
        assert_eq!(from_roman("M").unwrap(), 1000);
    }

    #[test]
    fn test_from_roman_subtractive() {
        assert_eq!(from_roman("IV").unwrap(), 4);
        assert_eq!(from_roman("IX").unwrap(), 9);
        assert_eq!(from_roman("XL").unwrap(), 40);
        assert_eq!(from_roman("XC").unwrap(), 90);
        assert_eq!(from_roman("CD").unwrap(), 400);
        assert_eq!(from_roman("CM").unwrap(), 900);
    }

    #[test]
    fn test_from_roman_complex() {
        assert_eq!(from_roman("XIV").unwrap(), 14);
        assert_eq!(from_roman("XLII").unwrap(), 42);
        assert_eq!(from_roman("MCMXCIX").unwrap(), 1999);
        assert_eq!(from_roman("MMXXIV").unwrap(), 2024);
        assert_eq!(from_roman("MMMCMXCIX").unwrap(), 3999);
    }

    #[test]
    fn test_from_roman_invalid() {
        assert!(from_roman("IIII").is_err());  // Too many I's
        assert!(from_roman("VV").is_err());    // V can't repeat
        assert!(from_roman("LL").is_err());    // L can't repeat
        assert!(from_roman("DD").is_err());    // D can't repeat
        assert!(from_roman("").is_err());      // Empty
        assert!(from_roman("ABC").is_err());   // Invalid chars
    }

    #[test]
    fn test_roundtrip() {
        for n in 1..=3999 {
            let roman = to_roman(n).unwrap();
            let back = from_roman(&roman).unwrap();
            assert_eq!(n, back, "Roundtrip failed for {}: {} -> {}", n, roman, back);
        }
    }

    #[test]
    fn test_looks_like_roman() {
        assert!(looks_like_roman("XIV"));
        assert!(looks_like_roman("MCMXCIX"));
        assert!(looks_like_roman("I"));
        assert!(!looks_like_roman(""));
        assert!(!looks_like_roman("ABC"));
        assert!(!looks_like_roman("X1V"));
    }
}
