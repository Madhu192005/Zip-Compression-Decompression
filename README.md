# 📦 ZIP Compression & Decompression CLI (Rust)

A command-line ZIP utility built in **Rust** implementing compression and decompression using the **DEFLATE algorithm**.  
Supports recursive directory compression while preserving folder structure.

---

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

---

## 🛠 Tech Stack
- **Rust** – Systems programming language
- **clap** – CLI argument parsing
- **flate2** – DEFLATE compression engine
- **zip** – ZIP archive reading/writing
- **Manual ZIP Encoder** – Custom binary structure handling
- **DFS Traversal** – Recursive directory traversal
---
## ⚙️ How To Run

### 1️⃣ Clone Repository

```bash
git clone <repository-url>
cd rust-pro
cargo build
cargo run -- compress file1.txt file2.txt -o output.zip
cargo run -- decompress output.zip
---
