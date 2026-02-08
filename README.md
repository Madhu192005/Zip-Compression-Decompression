Zip-Compression-Decompression (Rust CLI Tool)

A Rust-based Command Line Interface (CLI) tool to copy files, compress files, and decompress ZIP archives.

This project was built to understand low-level file handling and CLI tool development in Rust.

📌 Project Overview

This CLI tool allows users to:

Copy files

Compress single or multiple files into a .zip archive

Decompress ZIP files

The project focuses on understanding how real-world CLI utilities work internally.

🎯 What I Learned

File I/O in Rust

CLI argument parsing

Compression & decompression concepts

Folder traversal

Modular project structure

Error handling using Result

✨ Features

📂 Copy files

🗜️ Compress single files

🗜️ Compress multiple files into a ZIP archive

📦 Decompress ZIP files

🧩 Modular and clean project design

🛠 Tech Stack

Rust

clap – CLI argument parsing

zip – Compression & decompression

walkdir – Folder traversal

std::fs and std::io – File and I/O handling

📁 Project Structure
src/
├── main.rs
├── compress.rs
├── decompress.rs
├── file_io.rs
├── Cargo.toml

🚀 How to Run
Clone the Repository
git clone <repo>
cd rust-pro

Build the Project
cargo build

📌 Usage
1️⃣ Copy File
cargo run -- copy input.txt output.txt

2️⃣ Compress a Single File
cargo run -- compress file.txt -o file.zip

3️⃣ Compress Multiple Files
cargo run -- compress f1.txt,f2.txt -o files.zip

4️⃣ Decompress ZIP File
cargo run -- decompress archive.zip -o output

⚙️ Compression Details

Uses Deflate compression algorithm

Performs real compression (ZIP reduces file size)

Supports decompression of ZIP archives

🔮 Future Improvements

Custom archive format

Manual ZIP writing (without external crates)

Cross-platform binary releases

Progress indicators

Directory compression support

👤 Author

Madhusudhanan S
CSBS – SCE
