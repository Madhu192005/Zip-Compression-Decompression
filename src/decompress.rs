use std::io::Result;

pub fn decompress(input: &str, output: &str) -> Result<()> {
    println!("Decompressing {} → {}", input, output);
    Ok(())
}
