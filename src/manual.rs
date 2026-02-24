use std::fs::File;
use std::io::{Write,Seek,SeekFrom,Result};
use crc32fast::hash;

fn main()->std::io::Result<()>{
    let mut zip=File::create("my first manual zip")?;
    let filename=b"example.txt ";
    let data=b"Hello ZIP";
    let crc32=hash(data);
    let uncompsize=data.len() as u32;
    let compsize=uncompsize;//no compression

    //Local File header
    let local_header=zip.seek(SeekFrom::Current(0))? as u32;
    zip.write_all(&[0x50,0x4B,0x03,0x04])?;//sig
    zip.write_all(&[0x14,0x00])?; //ver=20
    zip.write_all(&[0x00,0x00])?; //general purpose bit flag
    zip.write_all(&[0x00,0x00])?;//store
    zip.write_all(&[0x00,0x00,0x00,0x00])?;
    zip.write_all(&[(crc32 & 0xFF)as u8,((crc32>>8)&0xFF)as u8 ,((crc32>>16)&0xFF)as u8,((crc32>>24)&0xFF)as u8,])?;
    zip.write_all(&[(compsize & 0xFF)as u8,((compsize >> 8)& 0xFF) as u8,((compsize>>16)& 0xFF)as u8,((compsize >> 24)& 0xFF)as u8,])?;
    zip.write_all(&[(uncompsize & 0xFF)as u8,((uncompsize >> 8)& 0xFF) as u8,((uncompsize>>16)& 0xFF)as u8,((uncompsize >> 24)& 0xFF)as u8,])?;
    let filenamelength=filename.len() as u16;//lemgth
    zip.write_all(&[(filenamelength & 0xFF)as u8,((filenamelength >> 8)& 0xFF) as u8,])?;
    zip.write_all(&[0x00,0x00])?;//extra field length
    zip.write_all(filename)?;
    zip.write_all(data)?; //the data(content)

    let cd_off=zip.seek(SeekFrom::Current(0))? as u32;
    zip.write_all(&[0x50,0x4B,0x01,0x02])?; //signature
    zip.write_all(&[0x14,0x00])?; //version made by
    zip.write_all(&[0x14,0x00])?; //version needed
    zip.write_all(&[0x14,0x00])?; //flags
    zip.write_all(&[0x00,0x00])?; //compression
    zip.write_all(&[0x00,0x00,0x00,0x00])?; //time/date

    zip.write_all(&[(crc32 & 0xFF) as u8,((crc32 >> 8) & 0xFF) as u8,((crc32 >> 16) & 0xFF) as u8,((crc32>>24)&0xFF) as u8,])?; //CRC32
    zip.write_all(&[(compsize & 0xFF) as u8,((compsize>> 8) & 0xFF) as u8,((compsize >> 16) & 0xFF) as u8,((compsize >>24)&0xFF) as u8,])?; //Compsize
    zip.write_all(&[(uncompsize & 0xFF) as u8,((uncompsize >> 8) & 0xFF) as u8,((uncompsize >> 16) & 0xFF) as u8,((uncompsize >>24)&0xFF) as u8,])?; //uncompressed size
    zip.write_all(&[(filenamelength & 0xFF) as u8,((filenamelength >> 8) & 0xFF) as u8])?; //uncompressed size
    zip.write_all(&[0x00,0x00,0x00,0x00])?; //extra+comment length
    zip.write_all(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00])?; //number + attributes
    zip.write_all(&[(local_header & 0xFF) as u8,((local_header >> 8) & 0xFF) as u8,((local_header >> 16) & 0xFF) as u8,((local_header>>24)&0xFF) as u8,])?; //uncompressed size
    zip.write_all(filename)?;

    //EOCD
    let end=zip.seek(SeekFrom::Current(0))? as u32;
    let cd=end-cd_off;
    zip.write_all(&[0x50, 0x4B, 0x05, 0x06])?; //sig
    zip.write_all(&[0x00, 0x00, 0x00, 0x00])?; // disks
    zip.write_all(&[0x01, 0x00, 0x01, 0x00])?;//entries
    zip.write_all(&[(cd & 0xFF) as u8,((cd >> 8) & 0xFF) as u8,((cd >> 16) & 0xFF) as u8,((cd >> 24) & 0xFF) as u8,])?;

    // central directory offset
    zip.write_all(&[(cd_off & 0xFF) as u8,((cd_off >> 8) & 0xFF) as u8,((cd_off >> 16) & 0xFF) as u8,((cd_off >> 24) & 0xFF) as u8,])?;
    zip.write_all(&[0x00, 0x00])?;
    println!("zip manually done!");
    Ok(())
}

Without helper
use std::fs::File;
use std::io::{Write,Seek,SeekFrom,Result};
use crc32fast::hash;

fn write_u16_le<W: Write>(w: &mut W, v: u16) -> Result<()> {
    w.write_all(&v.to_le_bytes())
}

fn write_u32_le<W: Write>(w: &mut W, v: u32) -> Result<()> {
    w.write_all(&v.to_le_bytes())
}
fn main()->std::io::Result<()>{
    let mut zip=File::create("my first manual zip")?;
    let filename=b"example.txt ";
    let data=b"Hello ZIP";
    let crc32=hash(data);
    let size=data.len() as u32;

    //Local file header
    let localh=zip.seek(SeekFrom::Current(0))? as u32;
    zip.write_all(&[0x50,0x4B,0x03,0x04])?;
    write_u16_le(&mut zip,20)?;
    write_u16_le(&mut zip,0)?;  // flags
    write_u16_le(&mut zip,0)?;  // compression (0 = store)
    write_u16_le(&mut zip,0)?;  // mod time
    write_u16_le(&mut zip,0)?; //mod date
    write_u32_le(&mut zip,crc32)?;
    write_u32_le(&mut zip,size)?;
    write_u32_le(&mut zip,size)?;
    write_u16_le(&mut zip,filename.len() as u16)?;
    write_u16_le(&mut zip,0)?; // extra length
    zip.write_all(data)?;
    //central dir
    let cdo =
    zip.seek(SeekFrom::Current(0))? as u32;
    zip.write_all(&[0x50, 0x4B, 0x01, 0x02])?;
    write_u16_le(&mut zip,20)?;
    write_u16_le(&mut zip,0)?;  // flags
    write_u16_le(&mut zip,0)?;  // compression (0 = store)
    write_u16_le(&mut zip,0)?;  // mod time
    write_u16_le(&mut zip,0)?; //mod date
    write_u32_le(&mut zip,crc32)?;
    write_u32_le(&mut zip,size)?;
    write_u32_le(&mut zip,size)?;
    write_u16_le(&mut zip,filename.len() as u16)?;
    write_u16_le(&mut zip,0)?; // extra length
    write_u16_le(&mut zip,0)?;
    write_u16_le(&mut zip,0)?;
    write_u16_le(&mut zip,0)?;
    write_u32_le(&mut zip,0)?;
    write_u32_le(&mut zip,localh)?;
    zip.write_all(filename)?;
    let end=zip.seek(SeekFrom::Current(0))? as u32;
    let cds=end-cdo;
    write_u16_le(&mut zip,0)?;
    write_u16_le(&mut zip,0)?;
    write_u16_le(&mut zip,1)?;
    write_u16_le(&mut zip,1)?;
    write_u32_le(&mut zip,cds)?;
    write_u32_le(&mut zip,cdo)?;
    write_u16_le(&mut zip,0)?; //comment
    println!("Manual");
    Ok(())
}
