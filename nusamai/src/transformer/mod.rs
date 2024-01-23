mod builder;
pub mod runner;
pub mod transform;

use crate::pipeline::{Feedback, Parcel, Receiver, Sender};

use nusamai_citygml::object::Entity;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransformError {
    #[error("transform error")]
    SendError(#[from] std::sync::mpsc::SendError<Parcel>),
}

pub trait Transformer: Send + Sync {
    fn run(&self, upstream: Receiver, downstream: Sender, feedback: &Feedback);
}

pub trait Transform {
    fn transform(&mut self, entity: Entity, out: &mut Vec<Entity>);
}
