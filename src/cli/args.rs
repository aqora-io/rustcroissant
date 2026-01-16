use clap::{Args, ColorChoice, Parser, Subcommand, ValueHint, builder::TypedValueParser};

use core::fmt::{self, Display, Formatter};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(about, version)]
pub struct CliArguments {
    /// The command to run.
    #[command(subcommand)]
    pub command: Command,

    /// Whether to use color. When set to `auto` if the terminal to supports it.
    #[clap(long, default_value_t = ColorChoice::Auto, default_missing_value = "always")]
    pub color: ColorChoice,
}

#[derive(Debug, Clone, Subcommand)]
#[command()]
pub enum Command {
    /// Verify a json+ld croissant file
    #[command(visible_alias = "v")]
    Verify(VerifyCommand),
    #[command(visible_alias = "g")]
    Generate(GenerateCommand),
}

#[derive(Debug, Clone, Parser)]
pub struct VerifyCommand {
    #[clap(flatten)]
    pub args: VerifyArgs,
}

#[derive(Debug, Clone, Parser)]
pub struct GenerateCommand {
    #[clap(flatten)]
    pub args: GenerateArgs,
}

/// Arguments for Verify.
#[derive(Debug, Clone, Args)]
pub struct VerifyArgs {
    /// Path to input croissant file. Use `-` to read input from stdin.
    #[clap(value_parser = input_value_parser(), value_hint = ValueHint::FilePath)]
    pub input: Input,
}

/// Arguments for Generate.
#[derive(Debug, Clone, Args)]
pub struct GenerateArgs {
    /// Path to output the croissant file schema. Use `-` to write to stdout.
    #[clap(value_parser = output_value_parser(), value_hint = ValueHint::FilePath)]
    pub output: Output,
}

#[derive(Debug, Clone)]
pub enum Input {
    Stdin,
    Path(PathBuf),
}

#[derive(Debug, Clone)]
pub enum Output {
    Stdout,
    Path(PathBuf),
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stdin => f.pad("stdin"),
            Self::Path(path) => path.display().fmt(f),
        }
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stdout => f.pad("stdout"),
            Self::Path(path) => path.display().fmt(f),
        }
    }
}

fn input_value_parser() -> impl TypedValueParser<Value = Input> {
    clap::builder::OsStringValueParser::new().try_map(|value| {
        if value.is_empty() {
            Err(clap::Error::new(clap::error::ErrorKind::InvalidValue))
        } else if value == "-" {
            Ok(Input::Stdin)
        } else {
            Ok(Input::Path(value.into()))
        }
    })
}

fn output_value_parser() -> impl TypedValueParser<Value = Output> {
    clap::builder::OsStringValueParser::new().try_map(|value| {
        if value.is_empty() {
            Err(clap::Error::new(clap::error::ErrorKind::InvalidValue))
        } else if value == "-" {
            Ok(Output::Stdout)
        } else {
            Ok(Output::Path(value.into()))
        }
    })
}
