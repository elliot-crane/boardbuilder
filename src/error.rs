use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid configuration: {0}")]
    InvalidConfigError(&'static str),

    #[error("Argument '{0}' is invalid - details: {1}")]
    InvalidArgumentError(&'static str, &'static str),

    #[error(transparent)]
    FilesystemError(std::io::Error),

    #[error(transparent)]
    UrlParseError(url::ParseError),

    #[error(transparent)]
    HttpError(reqwest::Error),

    #[error(transparent)]
    ImageError(image::ImageError),

    /// An error surfaced from the RIL image processing library
    #[error("RIL error: {0}")]
    RILError(ril::Error),
}
