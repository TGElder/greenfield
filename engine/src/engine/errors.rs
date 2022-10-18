use thiserror::Error;

#[derive(Error, Debug)]
#[error("Engine initialization error")]
pub struct InitializationError {
    #[from]
    source: Box<dyn std::error::Error>,
}
