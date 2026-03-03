# 📦 ZIP Compression & Decompression CLI (Rust)

A command-line ZIP utility built in **Rust** implementing compression and decompression using the **DEFLATE algorithm**.  
Supports recursive directory compression while preserving folder structure.

## 🚀 What It Does
This project:
- Compresses a single file
- Compresses multiple files
- Compresses entire folders recursively
- Decompresses ZIP archives
- Preserves original directory hierarchy
- Implements manual ZIP binary encoding
- Uses Depth-First Search (DFS) for traversal

It operates at the **byte level** and follows the ZIP archive specification including:
- Local File Headers
- Central Directory
- End of Central Directory (EOCD)

## 🛠 Tech Stack
- **Rust** – Systems programming language
- **clap** – CLI argument parsing
- **flate2** – DEFLATE compression engine
- **zip** – ZIP archive reading/writing
- **Manual ZIP Encoder** – Custom binary structure handling
- **DFS Traversal** – Recursive directory traversal

## ⚙️ How To Run
```bash
git clone <repository-url>
cd rust-pro
cargo build
cargo run -- compress file1.txt file2.txt -o output.zip
cargo run -- decompress output.zip
---
```
## 🧠 Internal Working
### 1️⃣ CLI Parsing
- Implemented using `clap`
- Supports:
  - `compress`
  - `decompress`
- Validates argument structure
- Generates help messages
- Handles incorrect usage errors

### 2️⃣ Input Validation
Before execution:
- Checks if input path exists
- Validates whether it is a file or directory
- Ensures read/write permissions
- Uses Rust’s `Result<T, E>` for safe error handling
  
### 3️⃣ Compression (Byte-Level)
#### Steps:
1. File opened using `std::fs::File`
2. Data read into `Vec<u8>`
3. ZIP writer initialized
4. Local File Header created
5. Data compressed using:
   - `flate2`
   - DEFLATE algorithm
6. Compressed bytes written into archive
7. Central Directory entry created

### 4️⃣ Decompression
#### Steps:
1. ZIP archive opened
2. Central Directory parsed
3. File entries iterated
4. For each file:
   - Compressed data read
   - DEFLATE decoding applied
   - Original content written back to disk
5. Directory structure reconstructed

### 5️⃣ Recursive Directory Traversal (DFS)

#### Initial Implementation
- Used `walkdir` crate
#### Final Implementation
- Removed `walkdir`
- Implemented manual DFS traversal

#### DFS Logic
- Traverse root directory
- Recursively enter subdirectories
- Add files while maintaining relative paths

