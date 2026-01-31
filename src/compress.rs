use std::fs::File;
use std::io::{Write,Result};
use zip::ZipWriter;
use zip::write::FileOptions;
use crate::file_io;
pub fn single(input:&str,output:&str)->Result<()>{
    let data=file_io::to_read(input)?;
    let org_size=data.len();
    let zipf=File::create(output)?;
    let mut zip=ZipWriter::new(zipf);
    let options=FileOptions::default();
    zip.start_file(input,options)?;
    zip.write_all(&data)?;
    zip.finish()?;
    println!("{}",org_size);
    println!("{}",output);
    Ok(())
}
