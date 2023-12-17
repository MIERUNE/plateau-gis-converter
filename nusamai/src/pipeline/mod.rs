pub mod feedback;
pub mod runner;
pub mod transform;

pub use feedback::*;
pub use runner::*;
pub use transform::*;

use std::sync::mpsc;

pub type Sender = mpsc::SyncSender<Parcel>;
pub type Receiver = mpsc::Receiver<Parcel>;

/// Message passing through pipeline stages
#[derive(Debug)]
pub struct Parcel {
    pub cityobj: nusamai_plateau::TopLevelCityObject,
}
