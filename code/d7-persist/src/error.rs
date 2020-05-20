//use failure_derive::*; -- using thiserror insted
use thiserror::*;
#[derive(Error, Debug)]
pub enum BlobError {
    #[error("No Room")]
    NoRoom,
    #[error("Too Big")]
    TooBig(u64),
    #[error("Item Not Fount")]
    NotFound,
    #[error("{}", 0)]
    Bincode(bincode::Error),
    #[error("{}", 0)]
    IO(std::io::Error),
    #[error("{}", 0)]
    Other(anyhow::Error),
}

impl From<anyhow::Error> for BlobError {
    fn from(fe: anyhow::Error) -> Self {
        BlobError::Other(fe)
    }
}

impl From<bincode::Error> for BlobError {
    fn from(fe: bincode::Error) -> Self {
        BlobError::Bincode(fe.into())
    }
}
impl From<std::io::Error> for BlobError {
    fn from(fe: std::io::Error) -> Self {
        BlobError::IO(fe.into())
    }
}
