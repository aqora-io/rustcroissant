use parquet::arrow::arrow_reader::{ArrowReaderMetadata, ArrowReaderOptions};
use std::{fs::File, io::BufWriter};
use thiserror::Error;

use crate::{
    cli::{GenerateArgs, GenerateCommand},
    convert::TransformerTrait,
};

#[derive(Debug, Error)]
pub enum GenerateError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Parquet(#[from] parquet::errors::ParquetError),
}

fn parquet(args: &GenerateArgs) -> Result<(), GenerateError> {
    let input = match &args.input {
        crate::cli::Input::Stdin => unimplemented!(),
        crate::cli::Input::Path(path) => path,
    };

    let output = match &args.output {
        crate::cli::Output::Stdout => unimplemented!(),
        crate::cli::Output::Path(path) => path,
    };

    let file = File::open(input)?;
    let metadata = ArrowReaderMetadata::load(&file, ArrowReaderOptions::default())?;

    let converter = crate::convert::Converter::new(
        args.split_name.to_owned(),
        args.source_id.to_owned(),
        args.suffix.to_owned(),
    );

    let record_set = crate::convert::ParquetTransformer::new(&metadata).transform(&converter);

    let dataset = crate::specs::Dataset {
        record_sets: crate::specs::OneOrMany::One(record_set),
        ..Default::default()
    };

    let file = File::create(output)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &dataset)?;

    Ok(())
}

pub fn generate(command: GenerateCommand) -> Result<(), GenerateError> {
    parquet(&command.args)
}
