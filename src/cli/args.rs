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
}

#[derive(Debug, Clone, Parser)]
pub struct VerifyCommand {
    #[clap(flatten)]
    pub args: VerifyArgs,
}

/// Arguments for Verify.
#[derive(Debug, Clone, Args)]
pub struct VerifyArgs {
    /// Path to input croissant file. Use `-` to read input from stdin.
    #[clap(value_parser = input_value_parser(), value_hint = ValueHint::FilePath)]
    pub input: Input,
}

#[derive(Debug, Clone)]
pub enum Input {
    Stdin,
    Path(PathBuf),
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Input::Stdin => f.pad("stdin"),
            Input::Path(path) => path.display().fmt(f),
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
