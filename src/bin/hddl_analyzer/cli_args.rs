use clap::{Parser, Subcommand, ValueEnum};
use hddl_analyzer::Transformation;

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
    /// Rewrite the input files in place, pretty-printed (HDDL only)
    Format(InputArgs),
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
    /// Transformations to apply, in the order given: conjunctive-preconditions,
    /// remove-equality-constraints, remove-types
    #[arg(long = "transform", value_parser = parse_transformation, value_delimiter = ',')]
    pub transform: Vec<Transformation>,
    /// Write the result to this file instead of stdout
    #[arg(short, long)]
    pub output_file: Option<String>,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Hddl,
    Json,
}

// the library's Transformation enum stays clap-free, so the flag values are
// mapped by hand
fn parse_transformation(name: &str) -> Result<Transformation, String> {
    match name {
        "conjunctive-preconditions" => Ok(Transformation::ConjunctivePreconditions),
        "remove-equality-constraints" => Ok(Transformation::RemoveEqualityConstraints),
        "remove-types" => Ok(Transformation::RemoveTypes),
        other => Err(format!(
            "unknown transformation '{other}' (expected conjunctive-preconditions, remove-equality-constraints, or remove-types)"
        )),
    }
}
