# Seroost

A lightning-fast document search engine built in Rust that indexes and searches through your documents using TF-IDF scoring.
**Note that credit goes to tsoding daily on YT for starting the project about 2 years ago. He created an XML search engine with a web interface and thought of extending it. I don't think he finished the project though, so i extended it from just xml and made it a CLI tool. Here's his yt channel: https://www.youtube.com/@TsodingDaily**

## Latest Updates (v0.1.2)

- 💻 **Code file parsing** with line number tracking for precise search results
- 🔍 **Enhanced programming language support** (Rust, Python, JavaScript, TypeScript, Java, C/C++, Go, PHP, Ruby, Swift, Kotlin)
- 📍 **Line-based search functionality** for accurate code location
- ⚡ **Multi-threaded indexing** utilizing all available CPU cores
- 📊 **Improved memory management** with file size limits
- 🎨 **Enhanced CLI output** with color-coded status messages
- 🔄 **Streaming file processing** for better performance
- 🔍 **Extended file format support** (PDF, TXT, XML, HTML, and source code files)

## Features

- 🚀 **Parallel processing** for faster document indexing
- 💻 **Source code indexing** with line number support for precise search results
- 🔍 **TF-IDF based search** with relevance scoring
- 📁 **Recursive directory traversal**
- ⚙️ **System-aware configuration storage**
- 🛠️ **User-friendly CLI** with detailed feedback
- 📝 **Multi-format support** including documents and source code

## Supported File Formats

- **Documents**: PDF, TXT, XML, HTML
- **Source Code**: Rust (.rs), Python (.py), JavaScript (.js), TypeScript (.ts), Java (.java), C/C++ (.c, .cpp, .h), Go (.go), PHP (.php), Ruby (.rb), Swift (.swift), Kotlin (.kt)

## Installation

### Prerequisites

- Rust and Cargo (1.70 or later)
- Linux-based system (tested on Ubuntu/Debian)

### Building from source

```bash
git clone https://github.com/parado-xy/seroost.git
cd seroost
cargo build --release

# Optional: Create a symlink to use from anywhere
sudo ln -s "$(pwd)/target/release/seroost" /usr/local/bin/
```

## Usage

### Indexing documents

```bash
# Index with default settings
seroost --index-path /path/to/documents index

# Index with custom file size limit (in MB)
seroost --index-path /path/to/documents --max-file-size 50 index
```

### Searching documents

```bash
# Simple search
seroost search "your query"

# Display usage guide
seroost usage
```

## Implementation Details

- **Multi-threaded Architecture:**

  - Separate threads for directory traversal
  - Worker thread pool for file processing
  - Dedicated thread for term frequency calculations

- **Memory Management:**

  - Configurable file size limits
  - Efficient string handling
  - Streaming file processing

- **Search Algorithm:**
  - TF-IDF scoring for relevance
  - Document frequency optimization
  - Top-K results ranking

## Project Structure

```
seroost/
├── src/
│   ├── main.rs          # Entry point and CLI handling
│   ├── lexer.rs         # Text tokenization
│   ├── parsers.rs       # File format parsers
│   ├── interact.rs      # Single-threaded implementation
│   └── interactives.rs  # Multi-threaded implementation
└── Cargo.toml
```

## Contributing

Contributions are welcome! Current focus areas:

1. Memory optimization for large document collections
2. Additional file format support
3. Search result caching
4. Query optimization
5. Unit test coverage

## License

MIT License

---

_Built with Rust 🦀 - Optimized for Performance_
