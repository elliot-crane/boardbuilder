use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    /// An error surfaced from the RIL image processing library
    #[error("RIL error: {0}")]
    RILError(ril::Error),
}
