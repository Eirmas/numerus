//! # Numerus++
//!
//! A Roman-themed mini programming language where variables can be declared
//! using either Arabic numerals or Roman numerals.
//!
//! ## Example
//! ```text
//! DECLARA X EST XV
//! DECLARA Y EST 10
//! DECLARA Z EST X ADDIUS Y
//! SCRIBE("Summa: " ADDIUS Z)
//! ```

pub mod banner;
pub mod error;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod roman;

// Re-export commonly used types
pub use error::NumerusError;
pub use interpreter::Interpreter;
pub use lexer::Lexer;
pub use parser::Parser;
pub use roman::{from_roman, to_roman};
