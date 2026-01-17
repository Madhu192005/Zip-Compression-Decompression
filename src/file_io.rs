use std::fs::File;
use std::io::{Read, Write, Result};
pub fn rf(path:&str)->Result<Vec<u8>> {
    let mut f=File::open(path)?;
    let mut buf=Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(buf)
}
pub fn wf(path:&str,d:&[u8])->Result<()> {
    let mut f=File::create(path)?;
    f.write_all(d)?;
    Ok(())
}

