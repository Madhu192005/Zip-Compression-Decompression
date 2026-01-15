use std::io::Result;

pub fn compress(input: &str, output: &str) -> Result<()> {
    println!("Compressing {} → {}", input, output);
    Ok(())
}
