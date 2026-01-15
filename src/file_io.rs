use std::fs::File;
use std::io::{Read, Write, Result};

/// Reads a file from disk and returns its contents as bytes
pub fn read_file(path: &str) -> Result<Vec<u8>> {
    // Open the file (OS gives us a file handle)
    let mut file = File::open(path)?;

    // Buffer to store file bytes
    let mut buffer = Vec::new();

    // Read entire file into memory
    file.read_to_end(&mut buffer)?;

    // Return the bytes
    Ok(buffer)
}

/// Writes bytes to a file on disk
pub fn write_file(path: &str, data: &[u8]) -> Result<()> {
    // Create or overwrite file
    let mut file = File::create(path)?;

    // Write all bytes
    file.write_all(data)?;

    Ok(())
}
