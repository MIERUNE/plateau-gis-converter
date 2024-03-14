use rayon::prelude::*;

use super::{builder::TransformBuilder, Transformer};
use crate::pipeline::{Feedback, Parcel, Receiver, Result, Sender};

// transforms: Vec<Box<dyn Transform>>,

#[derive(Default)]
pub struct MultiThreadTransformer<T: TransformBuilder> {
    builder: T,
}

impl<T: TransformBuilder> MultiThreadTransformer<T> {
    pub fn new(builder: T) -> Self {
        Self { builder }
    }
}

impl<T: TransformBuilder> Transformer for MultiThreadTransformer<T> {
    fn run(&self, upstream: Receiver, downstream: Sender, feedback: &Feedback) -> Result<()> {
        upstream.into_iter().par_bridge().try_for_each_init(
            || (self.builder.build(), Vec::default()),
            |(transform, buf), parcel| {
                feedback.ensure_not_canceled()?;

                // Apply transform to entity
                transform.transform(feedback, parcel.entity, buf);

                for entity in buf.drain(..) {
                    if downstream.send(Parcel { entity }).is_err() {
                        break;
                    }
                }
                Ok(())
            },
        )
    }
}
