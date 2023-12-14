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
use std::sync::mpsc::channel;

pub type Sender = mpsc::Sender<Percel>;
pub type Receiver = mpsc::Receiver<Percel>;

/// TODO: Move to citygml(plateau) crate
#[derive(Debug)]
pub struct Percel {
    pub dummy_value: i32,
    // pub cityobj: TopLevelCityObject
}
