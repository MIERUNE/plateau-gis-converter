//! The transformer stage that preprocesses the attributes and geometry of the entities.

mod builder;
mod runner;
pub mod transform;

pub use builder::*;
pub use runner::*;

use crate::pipeline::{Feedback, Parcel, Receiver, Result, Sender};

use nusamai_citygml::{object::Entity, schema::Schema};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransformError {
    #[error("transform error")]
    SendError(#[from] std::sync::mpsc::SendError<Parcel>),
}

pub trait Transformer: Send {
    fn run(&self, upstream: Receiver, downstream: Sender, feedback: &Feedback) -> Result<()>;
}

pub trait Transform: Send {
    /// Transform each entity
    fn transform(&mut self, entity: Entity, out: &mut Vec<Entity>);
    /// Transform the schema
    fn transform_schema(&self, schema: &mut Schema);
}
