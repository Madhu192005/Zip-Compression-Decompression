use std::fs::File;
use std::io::{Write, Seek, SeekFrom, Result};
use crc32fast::hash; 
//helper functions
// write 16-bit little endian
fn write_u16_le<W: Write>(w:&mut W,v:u16)->Result<()>{
    w.write_all(&[(v & 0xFF) as u8,((v >> 8) & 0xFF) as u8,])
}

// write 32-bit little endian
fn write_u32_le<W: Write>(w: &mut W, v: u32) -> Result<()> {
    w.write_all(&[(v & 0xFF) as u8,((v >> 8) & 0xFF) as u8,((v >> 16) & 0xFF) as u8,((v >> 24) & 0xFF) as u8,])
}
fn main()->Result<()> {
    let filename=b"hello.txt";
    let data=b"Hello ZIP from manual Rust!";
    let mut zip=File::create("manual.zip")?;
    let crc=hash(data);
    let size=data.len() as u32;
    let local_header_offset=zip.seek(SeekFrom::Current(0))? as u32;
    // signature PK\x03\x04
    zip.write_all(&[0x50, 0x4B, 0x03, 0x04])?;
    write_u16_le(&mut zip,20)?; // version needed
    write_u16_le(&mut zip,0)?;  // flags
    write_u16_le(&mut zip,0)?;  // compression = stored
    write_u16_le(&mut zip,0)?;  // mod time
    write_u16_le(&mut zip,0)?;  // mod date
    write_u32_le(&mut zip,crc)?;
    write_u32_le(&mut zip,size)?; // compressed size
    write_u32_le(&mut zip,size)?; // uncompressed size
    write_u16_le(&mut zip,filename.len() as u16)?;
    write_u16_le(&mut zip,0)?; // extra length
    zip.write_all(filename)?;
    zip.write_all(data)?;
    let central_dir_offset=zip.seek(SeekFrom::Current(0))? as u32;
    // signature PK\x01\x02
    zip.write_all(&[0x50, 0x4B, 0x01, 0x02])?;
    write_u16_le(&mut zip,20)?; // version made by
    write_u16_le(&mut zip,20)?; // version needed
    write_u16_le(&mut zip,0)?;  // flags
    write_u16_le(&mut zip,0)?;  // compression
    write_u16_le(&mut zip,0)?;  // time
    write_u16_le(&mut zip,0)?;  // date
    write_u32_le(&mut zip,crc)?;
    write_u32_le(&mut zip,size)?;
    write_u32_le(&mut zip,size)?;
    write_u16_le(&mut zip,filename.len() as u16)?;
    write_u16_le(&mut zip,0)?; // extra
    write_u16_le(&mut zip,0)?; // comment
    write_u16_le(&mut zip,0)?; // disk number
    write_u16_le(&mut zip,0)?; // internal attr
    write_u32_le(&mut zip,0)?; // external attr
    write_u32_le(&mut zip, local_header_offset)?;
    zip.write_all(filename)?;
    let end_pos=zip.seek(SeekFrom::Current(0))? as u32;
    let central_dir_size=end_pos-central_dir_offset;
    // signature PK\x05\x06
    zip.write_all(&[0x50,0x4B,0x05,0x06])?;
    write_u16_le(&mut zip,0)?; // disk number
    write_u16_le(&mut zip,0)?; // start disk
    write_u16_le(&mut zip,1)?; // entries on disk
    write_u16_le(&mut zip,1)?; // total entries
    write_u32_le(&mut zip,central_dir_size)?;
    write_u32_le(&mut zip,central_dir_offset)?;
    write_u16_le(&mut zip,0)?; // comment length
    println!("zip!");
    Ok(())
}
