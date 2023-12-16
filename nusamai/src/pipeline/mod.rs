pub mod feedback;
pub mod runner;
pub mod sink;
pub mod source;
pub mod transform;

pub use feedback::*;
pub use runner::*;
pub use sink::*;
pub use source::*;
pub use transform::*;

use std::sync::mpsc;

pub type Sender = mpsc::SyncSender<Percel>;
pub type Receiver = mpsc::Receiver<Percel>;

/// Message passing through pipeline stages
#[derive(Debug)]
pub struct Percel {
    pub cityobj: nusamai_plateau::TopLevelCityObject,
}
