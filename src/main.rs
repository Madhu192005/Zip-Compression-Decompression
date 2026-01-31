mod file_io;
use clap::{Parser, Subcommand};
use std::io::Result;
#[derive(Parser)]
#[command(name="hi")]
#[command(about="It`s just fun name")]
struct Cli{
    #[command(subcommand)]
    command:Command,
}
#[derive(Subcommand)]
enum Command{  
    Copy{
        input:String,
        output:String,
    },
}
fn main()->Result<()>{
    let cli=Cli::parse();
    match cli.command{
        Command::Copy { input, output }=>{
            if input==output{
                eprintln!("Error due to paths are same");
                return Ok(());
            }
            let data=file_io::to_read(&input)?;
            file_io::to_write(&output,&data)?;
            println!("File copied");
        }
    }
    Ok(())
}
