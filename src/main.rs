use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

mod lexer;
mod parser;
mod ast;
mod interpreter;
mod error;
mod stdlib;
mod compiler;
mod type_checker;

use error::TogError;

#[derive(Parser)]
#[command(name = "tog")]
#[command(about = "TOG Programming Language - Better than Python and Rust")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a TOG program
    Run {
        /// Path to the TOG source file
        file: PathBuf,
    },
    /// Compile a TOG program
    Build {
        /// Path to the TOG source file
        file: PathBuf,
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Format a TOG source file
    Fmt {
        /// Path to the TOG source file
        file: PathBuf,
    },
    /// Check syntax without running
    Check {
        /// Path to the TOG source file
        file: PathBuf,
    },
}

fn main() -> Result<(), TogError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file } => {
            let source = fs::read_to_string(&file)
                .map_err(|e| TogError::IoError(format!("Failed to read file: {}", e)))?;
            
            println!("Running TOG program: {}", file.display());
            
            // Lex
            let tokens = lexer::tokenize(&source)?;
            
            // Parse
            let ast = parser::Parser::parse(tokens)?;
            
            // Type check
            let mut type_checker = type_checker::TypeChecker::new();
            if let Err(e) = type_checker.check_program(&ast) {
                eprintln!("Type check warning: {}", e);
                // Continue anyway (gradual typing)
            }
            
            // Interpret
            interpreter::Interpreter::interpret(ast)?;
            
            Ok(())
        }
        Commands::Build { file, output } => {
            let source = fs::read_to_string(&file)
                .map_err(|e| TogError::IoError(format!("Failed to read file: {}", e)))?;
            
            println!("Building TOG program: {}", file.display());
            
            // Lex
            let tokens = lexer::tokenize(&source)?;
            
            // Parse
            let ast = parser::Parser::parse(tokens)?;
            
            // Compile using compiler backend
            let output_path = output.unwrap_or_else(|| {
                file.with_extension("exe")
            });
            
            // Use native C code generator as a working backend
            // This generates C code that can be compiled with GCC/Clang
            let opt_level = compiler::optimizer::OptimizationLevel::Standard;
            
            // Try native C backend first (works without external dependencies)
            let mut compiler = compiler::Compiler::new(
                compiler::backend::BackendType::NativeC,
                opt_level
            )?;
            
            match compiler.compile_to_file(ast, &output_path) {
                Ok(_) => {
                    println!("Build complete: {}", output_path.display());
                    println!("Generated C code. Compile with: gcc {} -o output", output_path.display());
                }
                Err(e) => {
                    // Fallback message
                    println!("Build error: {}", e);
                    println!("Note: For full native compilation, LLVM/Cranelift backends require additional dependencies.");
                }
            }
            
            Ok(())
        }
        Commands::Fmt { file } => {
            println!("Formatting TOG file: {}", file.display());
            println!("   (Formatter coming soon!)");
            Ok(())
        }
        Commands::Check { file } => {
            let source = fs::read_to_string(&file)
                .map_err(|e| TogError::IoError(format!("Failed to read file: {}", e)))?;
            
            println!("Checking syntax: {}", file.display());
            
            // Lex
            let tokens = lexer::tokenize(&source)?;
            
            // Parse
            let ast = parser::Parser::parse(tokens)?;
            
            // Type check
            let mut type_checker = type_checker::TypeChecker::new();
            type_checker.check_program(&ast)?;
            
            println!("Syntax and type check passed!");
            Ok(())
        }
    }
}

