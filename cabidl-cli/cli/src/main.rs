use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process;

use cabidl_diagram::Diagram;
use cabidl_parser::CabidlParser;
use cabidl_parser_impl::CabidlParserImpl;
use cabidl_filesystem_impl::RealFilesystem;

#[derive(Parser)]
#[command(name = "cabidl", version = "1.0.0")]
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
    }
}
