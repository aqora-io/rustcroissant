use schematic::schema::{JsonSchemaRenderer, SchemaGenerator};

use crate::{
    cli::{GenerateCommand, Output},
    specs::Dataset,
};

pub fn generate(command: GenerateCommand) -> miette::Result<()> {
    match command.args.output {
        Output::Stdout => Ok(()),
        Output::Path(path) => {
            let mut generator = SchemaGenerator::default();
            generator.add::<Dataset>();
            generator.generate(path.join("schema.json"), JsonSchemaRenderer::default())?;

            Ok(())
        }
    }
}
