mod file_io;
mod utils;
mod compress;
mod decompress;
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
    Compress{
        input:Vec<String>,
        #[arg(short, long)]
        output:String
    },
    Folder{
        input:String,
        #[arg(short,long)]
        output:String
    },
    Decompressed{
        zip:String,
        #[arg(short,long)]
        output:String,
    },
    CompressDir{
        dir:String,
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
        Command::Compress{input , output}=>{
            if input.len()==1{
                compress::single(&input[0],&output)?;
            }
            else{
                compress::multi(input,&output)?;
            }
        }
        Command::Folder {input,output}=>{
        compress::folder(&input,&output)?;
    }
        Command::Decompressed {zip, output }=>{
            decompress::decomp(&zip,&output)?;
        }
        Command::CompressDir { dir, output }=>{
            compress::folder(&dir,&output)?;
        }
    }
    Ok(())
}
