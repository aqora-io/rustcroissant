use crate::{convert::converter::Converter, specs::RecordSet};

pub trait TransformerTrait {
    fn transform(&self, convert: &Converter) -> RecordSet;
}
