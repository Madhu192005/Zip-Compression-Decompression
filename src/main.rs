mod file_io;

fn main() -> std::io::Result<()> {
    let data = file_io::read_file("input.txt")?;
    file_io::write_file("output.txt", &data)?;
    Ok(())
}
