use schematic::ConfigLoader;

use crate::cli::{Input, VerifyCommand};

pub fn verify(command: VerifyCommand) -> miette::Result<()> {
    match command.args.input {
        Input::Stdin => Ok(()),
        Input::Path(path) => {
            let context: crate::specs::Context = crate::specs::Context;

            ConfigLoader::<crate::specs::Dataset>::new()
                .file(path)?
                .load_with_context(&context)?;

            Ok(())
        }
    }
}
