pub mod feedback;
pub mod runner;

pub use feedback::*;
pub use runner::*;

use std::sync::mpsc;

pub type Sender = mpsc::SyncSender<Parcel>;
pub type Receiver = mpsc::Receiver<Parcel>;

/// Message passing through pipeline stages
#[derive(Debug)]
pub struct Parcel {
    pub entity: nusamai_citygml::object::Entity,
}
