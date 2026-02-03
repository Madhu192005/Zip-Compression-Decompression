use std::fs::{File,create_dir_all};
use std::io::{Result,copy};
use std::path::Path;
use zip::ZipArchive;

pub fn decomp(zip_path:&str,output_dir:&str)->Result<()>{
    let file=File::open(zip_path)?;
    let mut a=ZipArchive::new(file)?;
    for i in 0..a.len(){
        let mut e=a.by_index(i)?;
        let outpath=Path::new(output_dir).join(e.name());
        if e.is_dir(){
            create_dir_all(&outpath)?;
        }else{
            if let Some(parent)=outpath.parent(){
                create_dir_all(parent)?;
            }
            let mut out=File::create(&outpath)?;
            copy(&mut e,&mut out)?;
        }
    }
    println!("Decompressed{}",output_dir);
    Ok(())
}
