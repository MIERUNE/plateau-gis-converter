use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransformError {
    #[error("outside projection domain")]
    OutsideProjectionDomain,
}
