use nom::{error::Error as NomError, Err as NomErr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlaylistError {
    #[error("Failed to fetch the playlist: {0}")]
    FetchError(#[from] reqwest::Error),

    #[error("Failed to read the playlist file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Nom parsing error: {0:?}")]
    ParseError(NomErr<NomError<String>>),

    #[error("Parsing incomplete error: {0:?}")]
    Incomplete(String),

    #[error("Invalid location. Provide a valid URL or file path.")]
    InvalidLocation,
}
