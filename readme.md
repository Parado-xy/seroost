# Seroost

A lightning-fast document search engine built in Rust that indexes and searches through your documents using TF-IDF scoring.
**Note that credit goes to tsoding daily on YT for starting the project about 2 years ago. He created an XML search engine with a web interface and thought of extending it. I don't think he finished the project though, so i extended it from just xml and made it a CLI tool. Here's his yt channel: https://www.youtube.com/@TsodingDaily**
## Features

- 🚀 **Fast indexing** of documents in various formats (PDF, TXT, XML, HTML)
- 🔍 **TF-IDF based search** that understands document relevance
- 📁 **Recursive directory traversal** for processing entire document collections
- ⚙️ **Persistent configuration** to maintain search settings between runs
- 🛠️ **Command-line interface** for easy integration into workflows

## Installation

### Prerequisites

- Rust and Cargo (1.70 or later)

### Building from source

```bash
# Clone the repository
git clone https://github.com/parado-xy/seroost.git
cd seroost

# Build the project
cargo build --release

# Optional: Create a symlink to use from anywhere
sudo ln -s "$(pwd)/target/release/seroost" /usr/local/bin/
```

## Usage

### Indexing documents

First, index your documents directory:

```bash
# Using the binary
seroost --index-path /path/to/documents index

# Using cargo
cargo run -- --index-path /path/to/documents index
```

### Searching documents

After indexing, search through your documents:

```bash
# Search for specific terms
seroost search "your search query"

# Or with cargo
cargo run -- search "your search query"
```

### Command-Line Options

- `--index-path`: Specifies the directory containing documents to index
- `--max-file-size`: Sets maximum file size to index (in MB, default: 25)
- `--help`: Displays help information

### Built-in Commands

- `index`: Indexes the documents in the specified directory
- `search "query"`: Searches for documents matching the query
- `usage`: Displays a detailed walkthrough with colorized examples

### Getting Help

Seroost includes a built-in help system with practical examples:

```bash
# Display detailed usage walkthrough with examples
seroost usage

# This shows a step-by-step guide including:
# - Installation instructions
# - Sample document creation
# - Indexing examples with expected output
# - Search examples with expected results
```

## Supported File Types

- PDF documents
- Plain text files (.txt)
- XML files
- HTML documents

## Implementation Details

Seroost uses:
- Term Frequency-Inverse Document Frequency (TF-IDF) for relevance scoring
- JSON for storing the document index
- Custom tokenization for text processing
- Colorized terminal output for better readability

## Directory Structure

```
seroost/
├── src/
│   ├── main.rs       # Main application logic
│   ├── lexer.rs      # Text tokenization
│   └── parsers.rs    # File format parsers
├── indeces/          # Generated index files
│   ├── config.json   # Configuration
│   └── index.json    # Document index
└── Cargo.toml        # Dependencies
```

## Contributing

Contributions are welcome! Here are ways you can contribute:

1. Add support for more document formats
2. Improve search algorithms
3. Add unit tests
4. Enhance the CLI interface
5. Report bugs and suggest features

## License

MIT License

---

*Built with Rust and a passion for fast document search*
