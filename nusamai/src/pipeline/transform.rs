use super::{feedback, Percel};

pub trait Transformer: Send + Sync {
    fn transform(&self, obj: &mut Percel, feedback: &feedback::Feedback);
}
