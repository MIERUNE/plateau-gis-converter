//! The main pipeline for processing CityGML data
//!
//! [Source] => [Transformer] => [Sink]

pub mod feedback;
pub mod runner;

use std::sync::mpsc;

pub use feedback::*;
pub use nusamai_plateau::Entity;
pub use runner::*;
use thiserror::Error;

pub type Sender = mpsc::SyncSender<Parcel>;
pub type Receiver = mpsc::Receiver<Parcel>;

/// Message passing through the main processing pipeline
#[derive(Debug)]
pub struct Parcel {
    // Entity (Feature, Data, etc.)
    pub entity: Entity,
}

#[derive(Error, Debug)]
pub enum PipelineError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("CityGML parsing error: {0}")]
    ParseError(#[from] nusamai_citygml::ParseError),

    #[error("Conversion canceled")]
    Canceled,

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, PipelineError>;
