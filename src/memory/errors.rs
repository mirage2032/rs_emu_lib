use std::fmt::Display;
use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum FileError {
    #[error("File already exists: {0}")]
    FileExists(PathBuf),
    #[error("File does not exist: {0}")]
    FileDoesNotExist(PathBuf),
    #[error("Error creating file: {0}")]
    FileCreate(PathBuf),
    #[error("Error opening file: {0}")]
    FileOpen(PathBuf),
    #[error("Error reading from file")]
    ReadError,
    #[error("Error writing to file")]
    WriteError,
}

#[derive(Debug, Error, Clone)]
pub enum MemoryRWCommonError {
    #[error("Not mapped address: d:{0} h:{0:x}")]
    UnmappedAddress(u16),
    #[error("Device accessed at address over the bound: d:{0} h:{0:x}")]
    OutOfBounds(u16),
    #[error("{0}")]
    CustomError(String),
}

#[derive(Debug, Error, Clone)]
pub enum MemoryReadError{
    #[error("Memory read error: {0}")]
    CommonRWError(#[from] MemoryRWCommonError),
}

impl From<MemoryReadError> for String {
    fn from(err: MemoryReadError) -> String {
        format!("{}", err)
    }
}
#[derive(Debug, Error, Clone)]
pub enum MemoryWriteError{
    #[error("Memory write error: {0}")]
    CommonRWError(#[from] MemoryRWCommonError),
    #[error("Attempted to write memory at read only address: d:{0} h:{0:x}")]
    ReadOnly(u16),
}

impl From<MemoryWriteError> for String {
    fn from(err: MemoryWriteError) -> String {
        format!("{}", err)
    }
}

#[derive(Debug, Error, Clone)]
pub enum MemoryError {
    #[error("{0}")]
    MemWrite(#[from] MemoryWriteError),
    #[error("{0}")]
    MemRead(#[from] MemoryReadError),
    #[error("Memory file error: {0}")]
    FileError(#[from] FileError),
}
