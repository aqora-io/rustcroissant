use clap::Parser;

use crate::cli::CliArguments;

mod cli;
#[cfg(feature = "arrow")]
pub mod convert;

pub mod specs;

fn main() -> miette::Result<()> {
    let args = CliArguments::parse();

    match args.command {
        // TODO: manage this result
        cli::Command::Verify(verify) => cli::verify(verify),
        cli::Command::Generate(generate) => cli::generate(generate),
    }
}
