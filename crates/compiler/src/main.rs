use std::fs;
use std::path::PathBuf;

use cairo_lang_compiler::CompilerConfig;
use cairo_lang_sierra::program::Program;
use clap::{Parser, Subcommand};
use wasmable_cairo::compiler::{compile_cairo_project, run_cairo_project};

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
    Compile { path: PathBuf },
    Run { path: PathBuf },
    RunSierra { path: PathBuf },
}

fn main() {
    let cli = Cli::parse();
    let compiler_config = CompilerConfig {
        replace_ids: false,
        ..CompilerConfig::default()
    };
    let sierra_path = &PathBuf::from("target");
    fs::create_dir_all(&sierra_path).unwrap();
    let sierra_path = sierra_path.canonicalize().unwrap().join("sierra.json");
    // create file if not exists
    if !sierra_path.exists() {
        fs::write(&sierra_path, "").unwrap();
    }

    match &cli.command {
        Commands::Compile { path } => match compile_cairo_project(path, compiler_config) {
            Ok((program, _crate_id)) => {
                let program_json = serde_json::to_string(&program).unwrap();
                fs::write(&sierra_path, program_json).unwrap();
                println!("File saved at: {}", sierra_path.as_path().to_str().unwrap());
            }
            Err(e) => panic!("Error compiling Cairo project: {}", e),
        },
        Commands::Run { path } => match run_cairo_project(path, compiler_config) {
            Ok(result) => {
                println!("Result: {:#?}", result.value);
                // let program_json = serde_json::to_string(&program).unwrap();
                // fs::write(&sierra_path, program_json).unwrap();
                // println!("File saved at: {}", sierra_path.as_path().to_str().unwrap());
            }
            Err(e) => panic!("Error compiling Cairo project: {}", e),
        },
        Commands::RunSierra { path } => {
            match fs::read_to_string(path) {
                Ok(content) => {
                    let _program: Program = serde_json::from_str(&content).unwrap();
                }
                Err(e) => eprintln!("Error reading file: {}", e),
            }
            let (program, _) = compile_cairo_project(path, compiler_config).unwrap();
            println!("{}", serde_json::to_string(&program).unwrap());
        }
    };
}
