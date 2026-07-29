use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about = "HDDL parser, verifier, and transpiler")]
pub struct CLIArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Convert between HDDL and JSON, optionally applying transformations
    Convert(ConvertArgs),
    /// Semantically verify a program
    Verify(InputArgs),
    /// Show metadata about a domain
    Metadata(InputArgs),
}

#[derive(Parser)]
pub struct InputArgs {
    /// Path to the domain file or a whole-program JSON file; the format is
    /// inferred from the extension (.hddl or .json), anything else is rejected
    #[arg(index = 1)]
    pub input_path: String,
    #[arg(short, long)]
    pub problem_path: Option<String>,
}

#[derive(Parser)]
pub struct ConvertArgs {
    #[command(flatten)]
    pub input: InputArgs,
    /// Output format
    #[arg(long, value_enum, default_value = "json")]
    pub to: OutputFormat,
    #[arg(long)]
    pub untyped: bool,
    /// Write the result to this file instead of stdout
    #[arg(short, long)]
    pub output_file: Option<String>,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Hddl,
    Json,
}
