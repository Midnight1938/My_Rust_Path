pub mod args; // For Arguments
pub mod read; // For Reading
pub mod write; // For Writing
pub mod stats; // For Logging

const CHUNK_SIZE: usize = 16 * 1024; // A pre defined chunk size of 16KB