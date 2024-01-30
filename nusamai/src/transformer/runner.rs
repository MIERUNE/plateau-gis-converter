use super::builder::TransformBuilder;
use crate::pipeline::{Feedback, Parcel, Receiver, Sender};

use rayon::prelude::*;

use super::Transformer;

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
    fn run(&self, upstream: Receiver, downstream: Sender, feedback: &Feedback) {
        let _ = upstream.into_iter().par_bridge().try_for_each_init(
            || (self.builder.build(), Vec::default()),
            |(transform, buf), parcel| {
                if feedback.is_cancelled() {
                    log::info!("transformer cancelled");
                    return Err(());
                }

                // Apply transform to entity
                transform.transform(parcel.entity, buf);

                for entity in buf.drain(..) {
                    if downstream.send(Parcel { entity }).is_err() {
                        break;
                    }
                }
                Ok(())
            },
        );
    }
}
