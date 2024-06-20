//! The transformer stage that preprocesses the attributes and geometry of the entities.

mod builder;
mod rules;
mod runner;
pub mod transform;

pub use builder::*;
use nusamai_citygml::schema::Schema;
use nusamai_plateau::Entity;
pub use rules::*;
pub use runner::*;
use thiserror::Error;
pub use transform::{
    DataFlatteningOption, FeatureFlatteningOption, LodFilterMode, ObjectFlatteningOption,
};

use crate::pipeline::{Feedback, Parcel, Receiver, Result, Sender};

#[derive(Error, Debug, Clone)]
pub enum TransformError {
    #[error("transform error")]
    SendError(#[from] std::sync::mpsc::SendError<Parcel>),
}

pub trait Transformer: Send {
    fn run(&self, upstream: Receiver, downstream: Sender, feedback: &Feedback) -> Result<()>;
}

pub trait Transform: Send {
    /// Transform each entity
    fn transform(&mut self, feedback: &Feedback, entity: Entity, out: &mut Vec<Entity>);
    /// Transform the schema
    fn transform_schema(&self, schema: &mut Schema);
}
