use std::fs::File;
use std::io::{Read,Write,Result};
pub fn to_read(path:&str)->Result<Vec<u8>>{
    let mut f=File::open(path)?;
    let mut buf=Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(buf)
}
pub fn to_write(path:&str,data:&[u8])->Result<()>{
    let mut f=File::create(path)?;
    f.write_all(data)?;
    Ok(())
}
pub fn size(path:&str)->Result<u64>{
    Ok(std::fs::metadata(path)?.len())
}
