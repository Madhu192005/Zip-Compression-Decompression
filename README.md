# Zip-Compression-Decompression
Developed a CLI Tool for Zip Compression and Decompression in Rust
This is a Rust based CLI tool.It allows users to copy files,compress files & decompress files
I build it to understand the low level file handling CLI tools

A command line tool written in **Rust** to **copy,compress, and decompress files** using the ZIP format
This project was built to understand:
1.File I/O in rust
2.CLI argument parsing
3.Compression & decompression
4.Folder traversal
5.Structure
---
##Features
-Copy files
-compress single or multiple files into '.zip' files
-compress file
-decompress files
-modular design
---
Tech Stack
**Rust**
**clap** - CLI argument parsing
**zip** - compression & decompression
**walkdir** - folder traversal
Standard Rust Built-in 'std::fs' and 'std::io'
---

Project Structure:
src/
|-main.rs
|-compress.rs
|-decompress.rs
|-file_io.rs
|-cargo.toml

Run:
Clone repository
```bash
git clone <repo>
cd rust-pro

Build the project:
```bash
cargo build

Run using the commands
1.copy:
```bash
cargo run -- copy input.txt output.txt
2.Compress single file:
```bash
cargo run -- compress file.txt -o file.zip
3.Compress multiple file:
```bash
cargo run -- compress f1.txt,f2.txt -o files.zip
4.Decompress Zip:
```bash
cargo run -- decompressed archieve.zip -o output

**Compression details:
Uses Deflate algorithm
Real compression - zip reduces size
decompression

Learnings:
File Handling in rust
Working with crates
Error handling with result
CLI patterns
Understand compression

Future Improvements:
Custom archive format
Manual zip writing
Binary releases

Author:
Madhusudhanan S
CSBS - SCE
