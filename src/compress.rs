use std::fs::{File,read_dir};
use std::io::{Write,Result};
use zip::{ZipWriter,CompressionMethod};
use zip::write::FileOptions;
use flate2::Compression;
use flate2::write::DeflateEncoder;
use crate::file_io;
use walkdir::WalkDir;
use std::path::Path;

pub fn deflate(data:&[u8])->Result<Vec<u8>>{
    // Create deflate encoder with BEST compression
    let mut encoder=DeflateEncoder::new(Vec::new(),Compression::best());
    encoder.write_all(data)?;//write raw data into encoder
    Ok(encoder.finish()?) //finish compression and return comp bytes
}

fn traverse(base:&Path,current:&Path,zip:&mut ZipWriter<File>,options:FileOptions,)->Result<()>{
    for entry in read_dir(current)?{
        let entry=entry?;
        let path=entry.path();
        let relative =path.strip_prefix(base).unwrap();
        let name=relative.to_string_lossy();
        if path.is_dir(){
            zip.add_directory(format!("{}/",name),options)?;
            traverse(base,&path,zip,options)?;
}
else{
    println!("Adding:{}",name);
    let data=file_io::to_read(path.to_str().unwrap())?;
    let compressed = deflate(&data)?;
    zip.start_file(name.as_ref(), options)?;
    zip.write_all(&compressed)?;}
    }

Ok(())
}
pub fn single(input:&str,output:&str)->Result<()>{
    //read file into memory
    let data=file_io::to_read(input)?;
    let org_size=data.len();
    //compress data
    let comp=deflate(&data)?;
    let comp_size=comp.len();
    let zipf=File::create(output)?;
    let mut zip=ZipWriter::new(zipf);
    // ZIP options: Stored (NO zip-level compression)
    let options=FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    // Extract only file name (no full path)
    let name=Path::new(input).file_name().unwrap().to_string_lossy();
    zip.start_file(name,options)?; //Add file entry
    zip.write_all(&comp)?;//write already compressed bytes
    zip.finish()?;//finish zip
    println!("{}",org_size); //org size
    println!("{}",output); //zip
    println!("{}",comp_size); //comp size: in bytes
    Ok(())
}

pub fn multi(inputs:Vec<String>,output:&str)->Result<()>{
    let zipf=File::create(output)?;
    let mut zip=ZipWriter::new(zipf);
    let options=FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for input in inputs{
        let data=file_io::to_read(&input)?;
        let comp=deflate(&data)?;
        let name=Path::new(&input).file_name().unwrap().to_string_lossy();
        let comp_size=comp.len();
        zip.start_file(name, options)?;
        zip.write_all(&comp)?;
        println!("{}",input);
    }
    zip.finish()?;
    println!("{}",output);
    Ok(())
}
pub fn folder(input_dir:&str,output:&str)->Result<()>{
    let zip_file=File::create(output)?;
    let mut zip=ZipWriter::new(zip_file);
    let options = FileOptions::default()
        .compression_method(CompressionMethod::Stored);
    let root=Path::new(input_dir);
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()){
        let path=entry.path();
        let relative=match path.strip_prefix(root){
            Ok(p)if !p.as_os_str().is_empty()=>p,_=> continue,
        };
        let name=relative.to_string_lossy();
        if entry.file_type().is_dir() {
            zip.add_directory(format!("{}/", name), options)?;
        }else{
            let data=file_io::to_read(path.to_str().unwrap())?;
            let compressed=deflate(&data)?;
            zip.start_file(name.as_ref(),options)?;
            zip.write_all(&compressed)?;
        }
    }
    zip.finish()?;
    println!("Folder compressed:{}", output);
    Ok(())
}

