mod compress;
mod file_io;
use clap::{Parser, Subcommand};
use std::io::Result;
///cli structure
#[derive(Parser)]
#[command(name = "ziptool")]
#[command(about = "ZIP compression tool")]
struct Cli{
    #[command(subcommand)]
    command: Command,
}
//subcommands
#[derive(Subcommand)]
enum Command {
    Copy{
        input:String,
        output:String,
    },
    Compress{
        input:String,
        output:String,
    },
}
fn main()->Result<()> {
    let cli=Cli::parse();
    match cli.command{
        Command::Copy{input,output}=>{
            let d = file_io::rf(&input)?;
            file_io::wf(&output,&d)?;
            println!("Copied {} bytes",d.len());
        }
        Command::Compress {input,output}=>{
            compress::compress_single_file(&input,&output)?;
        }
    }
    Ok(())
}
