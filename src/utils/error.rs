use thiserror::Error;

#[derive(Error, Debug)]
pub enum RustQuantError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid data format")]
    InvalidFormat,
}
