use thiserror::Error;

#[derive(Error, Debug)]
#[error("Graphics initialization error")]
pub struct InitializationError {
    #[from]
    source: Box<dyn std::error::Error>,
}

#[derive(Error, Debug)]
#[error("Graphics rendering error")]
pub struct RenderError {
    #[from]
    source: Box<dyn std::error::Error>,
}

#[derive(Error, Debug)]
#[error("Graphics drawing error")]
pub struct DrawError {
    #[from]
    source: Box<dyn std::error::Error>,
}

#[derive(Error, Debug)]
#[error("Graphics screenshot error")]
pub struct ScreenshotError {
    #[from]
    source: Box<dyn std::error::Error>,
}

#[derive(Error, Debug)]
#[error("Graphics index error")]
pub struct IndexError {
    #[from]
    source: Box<dyn std::error::Error>,
}
