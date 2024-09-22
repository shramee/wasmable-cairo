use std::path::PathBuf;

use cairo_lang_compiler::CompilerConfig;
use clap::{Parser, Subcommand};
use wasmable_cairo::compiler::compile_cairo_project;

/// The main CLI application structure
#[derive(Parser)]
#[command(name = "MyApp")]
#[command(about = "A CLI app with subcommands", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Enum to define available subcommands
#[derive(Subcommand)]
enum Commands {
    Build { path: PathBuf },
}

fn main() {
    let cli = Cli::parse();
    let compiler_config = CompilerConfig {
        replace_ids: false,
        ..CompilerConfig::default()
    };

    match &cli.command {
        Commands::Build { path } => {
            let result = compile_cairo_project(path, compiler_config);
            match result {
                Ok(program) => println!("{:?}", serde_json::to_string(&program)),
                _ => {}
            }
        }
    };
}
