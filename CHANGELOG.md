# Changelog

All notable changes to Seroost will be documented in this file.

## [0.1.2] - 2025-09-14

### Added

- **Code file parsing support** with line number tracking
- Support for multiple programming languages (Rust, Python, JavaScript, TypeScript, Java, C/C++, Go, PHP, Ruby, Swift, Kotlin)
- `get_code_line_info` function for precise line-based search results in code files
- Line-numbered content indexing for better code search accuracy

### Enhanced

- Extended file format support to include common programming language files
- Improved search precision for source code with contextual line information

## [0.1.1] - 2025-04-20

### Added

- Multi-threaded architecture with automatic CPU core detection
- Color-coded CLI output for better user feedback
- System-aware configuration storage

### Changed

- Complete rewrite of indexing system for parallel processing
- Enhanced error reporting with colored output
- Restructured project architecture for better maintainability

## [0.1.0] - Initial Release

### Features

- Support for multiple file formats (PDF, TXT, XML, HTML)
- TF-IDF based search implementation
- Command-line interface
- Recursive directory traversal
- Simple configuration system
