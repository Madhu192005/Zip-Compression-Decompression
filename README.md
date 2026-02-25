#ZIP COMPRESSION & DECOMPRESSION
* Build a CLI based Tool for zip compression/decompression in Rust programming language using DEFLATE Algorithm supporting recursive directory compression with file structure
* The project is command line zip utility built in rust that can compress single file , multiple files , compress entire folder, decompress a zip archive back to original files
* It reads file , compress their data using the DEFLATE stores them in ZIP format can later extract them back

## Building steps :
1.CLI interface
2.File reading
3.COmpression
4.ZIP Packing
5.Decompression

## Flow:
![Diagram]("C:\Users\Admin\Pictures\Screenshots\Screenshot 2026-02-25 110625.png")

user command -> CLI -> Compress/decompress ->Compression engine(DEFLATE) ->Zip writer/reader ->File System

## Packages:
-zip
-flate2
-clap

-zip : Library used to create ZIP files, read ZIP archives , Zip rules
-Manual : zip binary format is written 

