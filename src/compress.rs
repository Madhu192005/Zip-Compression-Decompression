use std::fs::File;
use std::io::{Write, Result};
use zip::write::FileOptions;
use zip::{ZipWriter,CompressionMethod};
use crate::file_io;
/// Compress one file into a ZIP archive using DEFLATE
pub fn compress_single_file(input:&str,output:&str)->Result<()> {
    let d=file_io::rf(input)?;
    let original_size = d.len();
    let zip_file = File::create(output)?;
    let mut zip = ZipWriter::new(zip_file);
    let options=FileOptions::default().compression_method(CompressionMethod::Deflated).compression_level(Some(9)); //compression
    zip.start_file(input, options)?;
    zip.write_all(&d)?;
    zip.finish()?;
    println!("Original size:{} bytes", original_size);
    println!("ZIP created:{}",output);
    Ok(())
}
