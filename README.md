## ZIP COMPRESSION & DECOMPRESSION
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
![Flow-diagram]("flow-diagram.png")
<p align="center">
  <img src="flow-diagram.png" width="600">
</p>

user command -> CLI -> Compress/decompress ->Compression engine(DEFLATE) ->Zip writer/reader ->File System

## Packages:
-zip
-flate2
-clap

-zip : Library used to create ZIP files, read ZIP archives , Zip rules
-Manual : zip binary format is written 

## How does it work?
1.CLI PARSE: Application starts in main.rs where the CLI arguments are parsed using clap (library)
2.I/P Validation: Checks file type , read/write permissions,all operators use rust ownership model (Result<T,E>)
3.Compression(byte level): Reads file into byte buffer , Manual zip writer , Starts file entry with metadata and compression done by flate2(library) DEFLATE
4.Decompression: Opened by zip Archieve,CD is parsed recursively file entries iterated , decoded by deflate
5.Initialy recursion done by Walkdir(library) and now removed and manual code done by Depth-First-Search(DFS) tree method.

