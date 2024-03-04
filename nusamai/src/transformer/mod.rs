//! The transformer stage that preprocesses the attributes and geometry of the entities.

mod builder;
mod rules;
mod runner;
pub mod transform;

use thiserror::Error;

use nusamai_citygml::schema::Schema;
use nusamai_plateau::Entity;

use crate::pipeline::{Feedback, Parcel, Receiver, Result, Sender};
pub use builder::*;
pub use rules::*;
pub use runner::*;
pub use transform::{
    DataFlatteningOption, FeatureFlatteningOption, LodFilterMode, ObjectFlatteningOption,
};

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
