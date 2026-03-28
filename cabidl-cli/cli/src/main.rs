use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process;

use cabidl_ai_provider::AiProvider;
use cabidl_diagram::Diagram;
use cabidl_parser::CabidlParser;
use cabidl_parser_impl::CabidlParserImpl;
use cabidl_filesystem_impl::RealFilesystem;

const LONG_VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), " (spec 1.1)");
const SKILL_CONTENT: &str = include_str!("../../skill.md");

#[derive(Parser)]
#[command(name = "cabidl", version, long_version = LONG_VERSION)]
#[command(about = "Parse and validate CABIDL architecture specification files")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Resolve includes and output a single unified CABIDL document
    Read {
        /// Path to the CABIDL markdown file
        file: PathBuf,
    },
    /// Validate a CABIDL document (silent on success, errors on failure)
    Validate {
        /// Path to the CABIDL markdown file
        file: PathBuf,
    },
    /// Generate an architecture diagram from a CABIDL document
    Diagram {
        /// Path to the CABIDL markdown file
        file: PathBuf,
        /// Diagram output format
        #[arg(short = 't', long = "type", default_value = "graphviz")]
        diagram_type: String,
        /// Path to the output file
        #[arg(short = 'o', long = "output-file")]
        output_file: PathBuf,
    },
    /// Manage cabidl skills for AI tool providers
    Skill {
        #[command(subcommand)]
        command: SkillCommands,
    },
}

#[derive(Subcommand)]
enum SkillCommands {
    /// Install the cabidl skill to an AI tool provider
    Install {
        /// Target directory for skill installation (defaults to provider's default location)
        #[arg(short = 'd', long = "target-dir")]
        target_dir: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();
    let parser = CabidlParserImpl::new(Box::new(RealFilesystem));

    match cli.command {
        Commands::Read { file } => {
            match cabidl_parser_impl::resolve(&RealFilesystem, &file) {
                Ok(content) => print!("{}", content),
                Err(errors) => {
                    for e in &errors {
                        eprintln!("error: {}", e);
                    }
                    process::exit(1);
                }
            }
        }
        Commands::Validate { file } => {
            match parser.parse(&file) {
                Ok(system) => {
                    let errors = parser.validate(&system, &file.display().to_string());
                    if errors.is_empty() {
                        process::exit(0);
                    } else {
                        for e in &errors {
                            eprintln!("error: {}", e);
                        }
                        process::exit(1);
                    }
                }
                Err(errors) => {
                    for e in &errors {
                        eprintln!("error: {}", e);
                    }
                    process::exit(1);
                }
            }
        }
        Commands::Diagram { file, diagram_type, output_file } => {
            match parser.parse(&file) {
                Ok(system) => {
                    let errors = parser.validate(&system, &file.display().to_string());
                    if !errors.is_empty() {
                        for e in &errors {
                            eprintln!("error: {}", e);
                        }
                        process::exit(1);
                    }

                    let diagram = cabidl_diagram_impl::DiagramImpl::new(
                        vec![Box::new(cabidl_graphviz::GraphvizProvider)],
                        Box::new(RealFilesystem),
                    );

                    if let Err(e) = diagram.generate(&system, &diagram_type, &output_file) {
                        eprintln!("error: {}", e);
                        process::exit(1);
                    }
                }
                Err(errors) => {
                    for e in &errors {
                        eprintln!("error: {}", e);
                    }
                    process::exit(1);
                }
            }
        }
        Commands::Skill { command } => {
            match command {
                SkillCommands::Install { target_dir } => {
                    let provider = cabidl_claude_code::ClaudeCodeProvider::new(
                        Box::new(RealFilesystem),
                    );
                    if let Err(e) = provider.install_skill(
                        target_dir.as_deref(),
                        SKILL_CONTENT,
                    ) {
                        eprintln!("error: {}", e);
                        process::exit(1);
                    }
                }
            }
        }
    }
}
