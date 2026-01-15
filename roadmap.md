Roadmap - ZIP compression & decompression
* Day 1 - ZIP + DEFLATE
Topics: 1. what is zip?, Deflate, diff b/w .zip, .tar, .gz, Why rust for compression
Coding: create project, add dependencies, setup folder
Self practice: write main.rs, compress, de, utils.

* Day 2 - Rust File I/O
Topics: File::open, create, read, write, buffers, result & match
Coding: Read file, copy file, print file size
Self: read_file, write_file (path)

* Day 3
Topics: clap::parser, subcommands, flags
Coding: Implement CLI parsing
Self: Add validation, error messages

* Day 4 - ZipWriter
Topics: ZipWriter, start_file, zip metadata
Coding: compress single file into zip
Self: print original size, compressed size
I/P: pp.txt → O/P: o/p.zip

* Day 5 - DEFLATE
Topics: flate2, compression levels, why DEFLATE default in ZIP
Coding: use compression::best(), convert DEFLATE
Self: Test on text vs binary file

Project Scope
I/P: File, folder, ZIP, CLArg
O/P: Compressed ZIP file when I/P: text, compress
O/P: output.zip when I/P: myfolder/
O/P: extracted/ -> folder with org files I/P: archive.zip

* Day 6 - Multi-File Compression : Compress multiple files into one zip
Topics: Iterating arguments, File naming inside Zip
Coding: Add multiple files
Self: Handle duplicate names, Handle invalid files
O/P: 3 files added to output.zip

* Day 7 - Directory Traversal
Topics: walkdir, Relative vs absolute paths
Coding: Traverse folder, Add files recursively
Self: Preserve folder hierarchy

* Day 8 - Advanced Folder Compression
Topics: Ignoring directories, Handling empty folders, permissions [optional]
Coding: Fix edge cases
Self: Test deep folders

* Day 9 - ZIP Decompression
Topics: ZipArchive, by_index, is_dir()
Coding: Extract files, create directories
Self: overwrite protection

* Day 10 - Error Handling & Safety
Topics: Result, custom errors, Graceful failure
Coding: Replace unwrap, add messages
Self: Break your code intentionally

* Day 11 - Testing
Tasks: Test with large files, binary files & empty folders, measure speed
Self: Improve buffer size, clean code

* Day 12 - Final Polish
Tasks: Write README.md, Add usage examples, add screenshots, final refactor

Project Statement:
Built a CLI-based ZIP compression & decompression tool in Rust using DEFLATE algorithm, supporting recursive directory compression with preserved file structure.

Input: File 
Output: Compressed bytes inside ZIP

Compression:
Read → Compress → Write

Decompression:
Read → Decompress → Write


Simplified version:
Day 1  Project structure + ZIP/DEFLATE concepts
Day 2  Rust file handling (read/write, Result)
Day 3  CLI with clap
Day 4  Single-file ZIP compression
Day 5  Multi-file compression
Day 6  Directory traversal (walkdir)
Day 7  Directory compression
Day 8  ZIP decompression
Day 9  Error handling + Test
Day 10 Optimization + polish

Project Structure:
src/
 ├── main.rs
 ├── compress.rs
 ├── decompress.rs
 └── utils.rs

main.rs - CLI+routing
compress.rs - ZIP Compression
decompress.rs - ZIP extraction
utils.rs - Common helpers


🚀 You’re thinking correctly:
The fact you asked internal working means you’re thinking like a systems engineer, not a tutorial coder.







flate2:
* Rust implementation of DEFLATE algorithm
It provides:DEFLATE,zlib,gzip.
ZIP internally uses DEFLATE, and the zip crate uses flate2 under the hood.

Compression levels:
DEFLATE supports trade-off between speed and size.
In flate2:
Compression::fast()
Compression::default()
Compression::best()

| Level   | Meaning                     |
| ------- | --------------------------- |
| fast    | Less compression, very fast |
| default | Balanced                    |
| best    | Maximum compression, slower |

Why DEFLATE is default in ZIP?
| Reason           | Explanation            |
| ---------------- | ---------------------- |
| Universal        | Works on any file type |
| Fast             | Good compression speed |
| No patent        | Free & safe            |
| Stream-based     | Works while reading    |
| Widely supported | Every OS supports it   |

Coding — Use Compression::best():
use flate2::Compression;
use zip::write::FileOptions;

let options = FileOptions::default()
    .compression_method(zip::CompressionMethod::Deflated)
    .compression_level(Some(Compression::best()));

"ZIP will use DEFLATE at maximum compression effort."
Where compression actually happens
zip.write_all(&buffer)?;

Internally:
buffer
 ↓
LZ77
 ↓
Huffman
 ↓
compressed bits
 ↓
ZIP file

Comment explaining DEFLATE:

Add this comment in your code 👇

// DEFLATE = LZ77 (remove repetition) + Huffman coding (entropy compression)
// compression_level controls speed vs compression ratio

6.SELF TEST — TEXT vs BINARY (IMPORTANT):
SELF TEST — TEXT vs BINARY (IMPORTANT)
Test files
Text file
hello hello hello hello hello

Binary file
.jpg
.mp3
.exe

Run:
ziptool compress text.txt text.zip
ziptool compress image.jpg image.zip

Observe results
File type	Result	Why
Text	High compression	Repetition + patterns
Binary	Low compression	Already compressed

Key insight:
Compression works best on redundancy, not randomness.

7.WHY ZIP DOES NOT INVENT NEW COMPRESSION
Because:
Compression is math-heavy
DEFLATE is proven
Compatibility > novelty
Tools must interoperate
ZIP’s job = container, not algorithm research.



Decompression:
