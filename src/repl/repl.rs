use colored::*;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::banner::{print_banner, print_help, print_farewell};
use crate::error::format_error_with_context;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;

/// The Numerus++ Read-Eval-Print Loop
pub struct Repl {
    interpreter: Interpreter,
    editor: DefaultEditor,
}

impl Repl {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            interpreter: Interpreter::new(),
            editor: DefaultEditor::new()?,
        })
    }

    /// Run the REPL
    pub fn run(&mut self) {
        print_banner();

        loop {
            let prompt = format!("{} ", "NUMERUS>".bright_yellow().bold());

            match self.editor.readline(&prompt) {
                Ok(line) => {
                    let trimmed = line.trim();

                    // Skip empty lines
                    if trimmed.is_empty() {
                        continue;
                    }

                    // Handle special commands
                    if trimmed.eq_ignore_ascii_case("EXITUS") {
                        print_farewell();
                        break;
                    }

                    if trimmed.eq_ignore_ascii_case("AUXILIUM") {
                        print_help();
                        continue;
                    }

                    // Add to history
                    let _ = self.editor.add_history_entry(&line);

                    // Execute the line
                    self.execute_line(trimmed);
                }
                Err(ReadlineError::Interrupted) => {
                    println!("{}", "CTRL-C detectum. Scribe 'EXITUS' pro exire.".bright_yellow());
                }
                Err(ReadlineError::Eof) => {
                    print_farewell();
                    break;
                }
                Err(err) => {
                    eprintln!("{}: {:?}", "ERRATUM".bright_red(), err);
                    break;
                }
            }
        }
    }

    /// Execute a single line of Numerus++ code
    fn execute_line(&mut self, line: &str) {
        // Tokenize
        let mut lexer = Lexer::new(line);
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{}", format_error_with_context(line, &e).bright_red());
                return;
            }
        };

        // Parse
        let mut parser = Parser::new(tokens);
        let program = match parser.parse() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{}", format_error_with_context(line, &e).bright_red());
                return;
            }
        };

        // Execute each statement
        for statement in &program.statements {
            if let Err(e) = self.interpreter.execute(statement) {
                eprintln!("{}", format!("{}", e).bright_red());
            }
        }
    }
}

impl Default for Repl {
    fn default() -> Self {
        Self::new().expect("Failed to create REPL")
    }
}
