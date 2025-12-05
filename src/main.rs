//! Numerus++ - A Roman-themed mini programming language
//!
//! Usage:
//!   numerus              - Start the REPL
//!   numerus file.npp     - Execute a Numerus++ file
//!   numerus --check file - Check syntax without executing (JSON output)

use std::env;
use std::fs;
use std::process;

use colored::*;

use numerus::banner::print_mini_banner;
use numerus::error::format_error_with_context;
use numerus::interpreter::Interpreter;
use numerus::lexer::Lexer;
use numerus::parser::Parser;
use numerus::repl::Repl;
use numerus::NumerusError;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for --check mode
    if args.len() >= 2 && args[1] == "--check" {
        if args.len() < 3 {
            eprintln!("Usage: numerus --check <file.npp>");
            process::exit(1);
        }
        let filename = &args[2];
        match fs::read_to_string(filename) {
            Ok(source) => {
                check_program(&source);
            }
            Err(e) => {
                // Output file read error as JSON
                println!(
                    r#"{{"diagnostics":[{{"line":1,"column":1,"end_line":1,"end_column":1,"severity":"error","message":"Cannot read file: {}"}}]}}"#,
                    e.to_string().replace('"', "\\\"")
                );
                process::exit(1);
            }
        }
        return;
    }

    match args.len() {
        1 => {
            // REPL mode
            match Repl::new() {
                Ok(mut repl) => repl.run(),
                Err(e) => {
                    eprintln!("{}: {}", "ERRATUM".bright_red(), e);
                    process::exit(1);
                }
            }
        }
        2 => {
            // File execution mode
            let filename = &args[1];

            // Check for help flag
            if filename == "--help" || filename == "-h" || filename == "AUXILIUM" {
                print_usage();
                return;
            }

            // Check for version flag
            if filename == "--version" || filename == "-v" {
                print_version();
                return;
            }

            match fs::read_to_string(filename) {
                Ok(source) => {
                    print_mini_banner();
                    if let Err(e) = run_program(&source) {
                        eprintln!("{}", format_error_with_context(&source, &e).bright_red());
                        process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!(
                        "{}: Non possum legere file '{}': {}",
                        "ERRATUM".bright_red(),
                        filename,
                        e
                    );
                    process::exit(1);
                }
            }
        }
        _ => {
            print_usage();
            process::exit(1);
        }
    }
}

/// Run a complete Numerus++ program
fn run_program(source: &str) -> Result<(), numerus::NumerusError> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;

    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;

    let mut interpreter = Interpreter::new();
    interpreter.run(&program)?;

    Ok(())
}

/// Check program syntax and output diagnostics as JSON
fn check_program(source: &str) {
    let mut diagnostics = Vec::new();

    // Try lexing
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            diagnostics.push(error_to_diagnostic(&e, source));
            print_diagnostics(&diagnostics);
            return;
        }
    };

    // Try parsing
    let mut parser = Parser::new(tokens);
    if let Err(e) = parser.parse() {
        diagnostics.push(error_to_diagnostic(&e, source));
    }

    print_diagnostics(&diagnostics);
}

/// Convert a NumerusError to a diagnostic JSON object
fn error_to_diagnostic(error: &NumerusError, source: &str) -> String {
    let (line, column, end_line, end_column) = match error.span() {
        Some(span) => (span.line, span.column, span.line, span.column + (span.end - span.start).max(1)),
        None => {
            // Try to extract line info from error variants without span
            match error {
                NumerusError::UnexpectedCharacter { line, column, .. } => {
                    (*line, *column, *line, *column + 1)
                }
                NumerusError::UnterminatedString { line } => {
                    (*line, 1, *line, source.lines().nth(line.saturating_sub(1)).map(|l| l.len()).unwrap_or(1))
                }
                _ => (1, 1, 1, 1),
            }
        }
    };

    let message = error.to_string().replace('"', "\\\"").replace('\n', " ");

    format!(
        r#"{{"line":{},"column":{},"end_line":{},"end_column":{},"severity":"error","message":"{}"}}"#,
        line, column, end_line, end_column, message
    )
}

/// Print diagnostics as JSON
fn print_diagnostics(diagnostics: &[String]) {
    println!(r#"{{"diagnostics":[{}]}}"#, diagnostics.join(","));
}

fn print_usage() {
    println!("{}", "NUMERUS++ - Lingua Programmandi Romana".bright_yellow().bold());
    println!();
    println!("Usus:");
    println!("  numerus              - Incipe REPL (modus interactivus)");
    println!("  numerus <file.npp>   - Exsequi file Numerus++");
    println!("  numerus --help       - Monstra hoc auxilium");
    println!("  numerus --version    - Monstra versionem");
    println!();
    println!("Exemplum:");
    println!("  {} example.npp", "numerus".green());
    println!();
}

fn print_version() {
    println!("{} {}", "NUMERUS++".bright_yellow().bold(), env!("CARGO_PKG_VERSION"));
    println!("Roma Aeterna Est!");
}
