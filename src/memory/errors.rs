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
pub enum MemWriteError {
    #[error("Error writing to address: d:{0} h:{0:x}. {1}")]
    Write(usize, &'static str),
    #[error(
        "Attempted to write to address: d:{0} h:{0:x} beyond the end of mapped memory. Skipping..."
    )]
    EndOfMem(usize),
}

#[derive(Debug, Error, Clone)]
pub enum MemoryError {
    #[error("File error: {0}")]
    File(#[from] FileError),
    #[error("Memory write error: {0}")]
    MemWrite(#[from] MemWriteError),
    #[error("Error reading from address: d:{0} h:{0:x}. {1}")]
    MemRead(usize, &'static str),
}
