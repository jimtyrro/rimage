use std::collections::BTreeMap;

use clap::ArgMatches;
use zune_image::traits::OperationsTrait;

use super::{quantization::Quantization, resize::Resize};

pub struct PreprocessorPipeline(BTreeMap<usize, Box<dyn OperationsTrait>>);

impl PreprocessorPipeline {
    pub fn from_matches(matches: &ArgMatches) -> Self {
        let mut pipeline: BTreeMap<usize, Box<dyn OperationsTrait>> = BTreeMap::new();

        if let Some(resize) = Resize::from_matches(matches) {
            resize.for_each(|(resize, index)| {
                pipeline.insert(index, Box::new(resize));
            });
        }

        if let Some(quantization) = Quantization::from_matches(matches) {
            quantization.for_each(|(quantization, index)| {
                pipeline.insert(index, Box::new(quantization));
            });
        }

        Self(pipeline)
    }
}
