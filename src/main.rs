use clap::Parser;

use crate::cli::CliArguments;

pub mod cli;

#[cfg(feature = "convert")]
pub mod convert;
pub mod specs;

fn main() {
    let args = CliArguments::parse();

    match args.command {
        // TODO: manage this result
        cli::Command::Verify(verify) => cli::verify(verify).expect("should work"),
        cli::Command::Generate(generate) => cli::generate(generate).expect("should work"),
        cli::Command::Schema(schema) => cli::schema(schema).expect("should work"),
    }
}
