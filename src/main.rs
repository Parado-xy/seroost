use clap::{Parser, Subcommand};
use colored::*; // Add this import
use serde_json;
use std::collections::HashMap;
use std::fs; // Get the file system.
use std::io;
use std::path::{Path, PathBuf};
use std::process;

// Import Modules.
mod lexer;
mod parsers;

// Define 2 types.
type TermFreq = HashMap<String, usize>;
type TermFreqIndex = HashMap<PathBuf, TermFreq>;

// Define CLI Interface.
#[derive(Parser)]
#[command(name = "seroost")]
#[command(version = "1.0.0")]
#[command(about = "Searches the content of documents", long_about = None)]
struct Cli {
    /// Pass an index path.
    /// This path will be saved and used should the search command be used.
    #[arg(short, long)]
    index_path: Option<String>,

    #[command(subcommand)]
    command: Option<AppCommands>,

    #[arg(short, long, default_value = "25")]
    max_file_size: u64,
}

#[derive(Subcommand)]
enum AppCommands {
    /// Indexes a directory to enable searching functionality.
    Index,

    /// Searches the Indexed documents for a document matching your description.
    Search {
        /// Term to search for
        #[arg(required = true)]
        term: String,
    },

    /// Displays detailed usage instructions and examples
    Usage,
}

fn main() -> Result<(), parsers::GlobalError> {
    // Parse CLI args.
    let cli = Cli::parse();
    let index_path: String;
    let max_file_size: u64 = cli.max_file_size * 1024u64 * 1024u64;

    // See if an index path was provided.
    if let Some(path) = &cli.index_path {
        // Create a configuration file.
        const CONFIG_PATH: &str = "./indeces/config.json";
        if let Some(parent) = Path::new(CONFIG_PATH).parent() {
            fs::create_dir_all(parent)?;
        }
        println!("Creating configuration file @: {CONFIG_PATH}...");
        let config_file = fs::File::create(CONFIG_PATH)?;
        let mut configuration = HashMap::<String, String>::new();
        configuration.insert("index_path".to_string(), path.clone());
        serde_json::to_writer(config_file, &configuration)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        // Keep index_path in memory
        index_path = path.to_string();
    } else {
        // Open the file in read-only mode with buffer.
        const CONFIG_PATH: &str = "./indeces/config.json";
        if !Path::new(CONFIG_PATH).exists() {
            eprintln!("Error: No index path provided and no saved configuration found.");
            eprintln!("Please run the program with --index-path option first:");
            eprintln!("    cargo run -- --index-path /path/to/documents index");
            process::exit(1);
        }
        let file = fs::File::open(CONFIG_PATH)?;
        let reader = std::io::BufReader::new(file);

        // Read the JSON contents of the file as an instance of `User`.
        let config: HashMap<String, String> =
            serde_json::from_reader(reader).expect("Error Parsing The Json configuration file");
        index_path = config
            .get("index_path")
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No index_path in config file"))?
            .clone();
    }

    match &cli.command {
        Some(AppCommands::Index) => {
            index_documents(&index_path, max_file_size)?;
        }
        Some(AppCommands::Search { term }) => {
            search_documents(term)?;
        }
        Some(AppCommands::Usage) => {
            display_usage()?;
        }
        None => {
            println!("No command provided. Use --help for usage information.");
            println!("Or try: {} for detailed examples", "seroost usage".green());
        }
    }

    Ok(())
}

fn process(
    dir_path: &str,
    term_frequency_index: &mut TermFreqIndex,
    max_file_size: u64,
) -> Result<(), parsers::GlobalError> {
    let dir_content = fs::read_dir(dir_path);

    match dir_content {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();

                // Handle directory recursion - pass the same index to accumulate results
                if path.is_dir() {
                    process(
                        path.to_str().unwrap_or(dir_path),
                        term_frequency_index,
                        max_file_size,
                    )?;
                    continue;
                }
                // Add inside your process function before attempting to read files
                if let Ok(metadata) = fs::metadata(&path) {
                    let file_size = metadata.len();
                    if file_size > max_file_size {
                        println!(
                            "{} {:?} ({:.2}MB)",
                            "Skipping large file:".yellow(),
                            path,
                            file_size as f64 / (1024.0 * 1024.0)
                        );
                        continue;
                    }
                }

                // Process files as you were doing before...
                let mut content: Vec<char> = Vec::new();
                match path.extension() {
                    Some(ext) => match ext.to_string_lossy().to_lowercase().as_str() {
                        "pdf" => {
                            content = parsers::read_entire_pdf_file(&path)?
                                .chars()
                                .collect::<Vec<_>>();
                        }
                        "txt" => {
                            content = parsers::read_entire_txt_file(&path)?
                                .chars()
                                .collect::<Vec<_>>();
                        }
                        "xml" | "xhtml" => {
                            content = parsers::read_entire_xml_file(&path)?
                                .chars()
                                .collect::<Vec<_>>();
                        }
                        "html" | "htm" => {
                            content = parsers::read_entire_html_file(&path)?
                                .chars()
                                .collect::<Vec<_>>();
                        }
                        _ => {
                            eprintln!("Error: do not know how to process file: {path:?}. Skipping file...");
                            continue;
                        }
                    },
                    None => {
                        eprintln!(
                            "Error: do not know how to process file: {path:?}. Skipping file..."
                        );
                        continue;
                    }
                }

                // Create a hashmap/table for storing the term frequency of each document.
                let mut term_frequency = TermFreq::new();
                let lexer = lexer::Lexer::new(&content);
                for token in lexer {
                    let term = token;

                    // Check if the term already exists in the HashMap/Table.
                    if let Some(count) = term_frequency.get_mut(&term) {
                        *count += 1; // Increment the frequency count of the word in the doc if it already exists.
                    } else {
                        // `move` the term into the hashmap/table because it gets dropped after this point regardless.
                        term_frequency.insert(term, 1); // Start The Term Count Up with a 1
                    }
                }
                // Create a vector containing (term, frequency) sorted in reverse.
                let mut stats = term_frequency.iter().collect::<Vec<_>>();
                stats.sort_by_key(|(_, f)| *f);
                stats.reverse();
                println!("{} {}", "Indexing:".blue(), path.to_string_lossy().green());

                // Add a file and its term frequency to the term_frequeny_index table.
                term_frequency_index.insert(path.to_path_buf(), term_frequency);
            }
        }
        Err(_) => {
            eprintln!("Error could not read dir: {dir_path}");
        }
    }

    Ok(())
}

fn index_documents(dir_path: &str, max_file_size: u64) -> Result<(), parsers::GlobalError> {
    // Create a new empty index first
    let mut term_frequency_index = TermFreqIndex::new();

    // Process documents and populate the index
    println!(
        "{} {}",
        "Indexing directory:".green().bold(),
        dir_path.blue()
    );
    process(dir_path, &mut term_frequency_index, max_file_size)?;

    // Save the complete index only once after all processing is done
    const INDEX_PATH: &str = "./indeces/index.json";
    if let Some(parent) = Path::new(INDEX_PATH).parent() {
        fs::create_dir_all(parent)?;
    }
    println!("{} {}", "Saving index to:".green(), INDEX_PATH.blue());
    let index_file = fs::File::create(INDEX_PATH)?;
    serde_json::to_writer(index_file, &term_frequency_index)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    println!(
        "{} {} {}",
        "Successfully indexed".green().bold(),
        term_frequency_index.len().to_string().yellow().bold(),
        "documents".green().bold()
    );

    Ok(())
}

fn search_documents(query: &str) -> Result<(), parsers::GlobalError> {
    // Load the index
    const INDEX_PATH: &str = "./indeces/index.json";
    if !Path::new(INDEX_PATH).exists() {
        eprintln!(
            "{}",
            "Error: index file not found. Please run index first."
                .red()
                .bold()
        );
        return Ok(());
    }

    let index_file = fs::File::open(INDEX_PATH)?;
    let reader = std::io::BufReader::new(index_file);
    let term_frequency_index: TermFreqIndex = serde_json::from_reader(reader)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    println!("{}", "Loading search index...".blue());

    // Tokenize the search query
    let query_chars = query.chars().collect::<Vec<_>>();
    let lexer = lexer::Lexer::new(&query_chars);
    let query_terms: Vec<String> = lexer.collect();

    if query_terms.is_empty() {
        println!("{}", "No valid search terms found.".yellow());
        return Ok(());
    }

    // Calculate document frequencies for IDF
    let total_docs = term_frequency_index.len() as f64;
    let mut document_frequency: HashMap<String, usize> = HashMap::new();

    // Count documents containing each term
    for term in &query_terms {
        for (_, doc_term_freq) in &term_frequency_index {
            if doc_term_freq.contains_key(term) {
                *document_frequency.entry(term.clone()).or_insert(0) += 1;
            }
        }
    }

    // Calculate TF-IDF scores for each document
    let mut document_scores: HashMap<PathBuf, f64> = HashMap::new();

    for (doc_path, term_freq) in &term_frequency_index {
        // Calculate total terms in document for TF normalization
        let total_terms: usize = term_freq.values().sum();
        let total_terms = total_terms as f64;

        // Calculate TF-IDF score for each query term in this document
        let mut score = 0.0;
        for term in &query_terms {
            if let Some(&term_count) = term_freq.get(term) {
                // TF = term count / total terms
                let tf = term_count as f64 / total_terms;

                // IDF = log(total docs / docs with term)
                let doc_freq = document_frequency.get(term).unwrap_or(&1);
                let idf = (total_docs / *doc_freq as f64).ln();

                // TF-IDF = TF * IDF
                score += tf * idf;
            }
        }

        if score > 0.0 {
            document_scores.insert(doc_path.clone(), score);
        }
    }

    // Sort documents by score
    let mut ranked_docs: Vec<(&PathBuf, &f64)> = document_scores.iter().collect();
    ranked_docs.sort_by(|(_, score1), (_, score2)| {
        score2
            .partial_cmp(score1)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Display results with color
    println!(
        "{} {}",
        "Search results for:".green().bold(),
        query.white().on_blue().bold()
    );

    if ranked_docs.is_empty() {
        println!("{}", "No matching documents found.".yellow());
    } else {
        println!("{}", "═".repeat(60).cyan());
        for (i, (path, score)) in ranked_docs.iter().take(10).enumerate() {
            let filename = path.file_name().unwrap_or_default().to_string_lossy();

            // Format ranking number
            let rank = format!("{}.", i + 1).yellow().bold();

            // Format path with colorized filename
            let path_str = path.to_string_lossy();
            let colorized_path = path_str.replace(&*filename, &filename.green().bold().to_string());

            // Format score
            let score_str = format!("Score: {:.5}", score).bright_blue();

            println!("{} {} ({})", rank, colorized_path, score_str);
        }
        println!("{}", "═".repeat(60).cyan());
    }

    Ok(())
}

fn display_usage() -> Result<(), parsers::GlobalError> {
    println!("{}", "═".repeat(80).cyan());
    println!(
        "{}",
        "SEROOST DETAILED USAGE GUIDE".bold().green().underline()
    );
    println!("{}", "═".repeat(80).cyan());
    println!();

    // Installation section
    println!("{}", "INSTALLATION".yellow().bold());
    println!("Clone and build the repository:");
    println!(
        "  {} git clone https://github.com/parado-xy/seroost.git",
        "$".bright_black()
    );
    println!("  {} cd seroost", "$".bright_black());
    println!("  {} cargo build --release", "$".bright_black());
    println!();
    println!("Make the binary executable from anywhere (optional):");
    println!(
        "  {} sudo ln -s \"$(pwd)/target/release/seroost\" /usr/local/bin/",
        "$".bright_black()
    );
    println!();

    // Sample documents section
    println!("{}", "CREATING SAMPLE DOCUMENTS".yellow().bold());
    println!("Create a sample document directory for testing:");
    println!("  {} mkdir -p ~/documents/samples", "$".bright_black());
    println!("  {} cd ~/documents/samples", "$".bright_black());
    println!(
        "  {} echo \"Rust is a systems programming language focused on safety.\" > rust.txt",
        "$".bright_black()
    );
    println!(
        "  {} echo \"Python is known for its simplicity and readability.\" > python.txt",
        "$".bright_black()
    );
    println!();

    // Indexing section
    println!("{}", "INDEXING DOCUMENTS".yellow().bold());
    println!("Index your documents directory:");
    println!(
        "  {} seroost --index-path ~/documents/samples index",
        "$".bright_black()
    );
    println!();
    println!("{}", "Expected output:".bright_blue());
    println!("  Creating configuration file @: ./indeces/config.json...");
    println!(
        "  {}Indexing directory:{} ~/documents/samples",
        "".green().bold(),
        "".blue()
    );
    println!(
        "  {}Indexing:{} ~/documents/samples/rust.txt",
        "".blue(),
        "".green()
    );
    println!(
        "  {}Indexing:{} ~/documents/samples/python.txt",
        "".blue(),
        "".green()
    );
    println!(
        "  {}Saving index to:{} ./indeces/index.json",
        "".green(),
        "".blue()
    );
    println!(
        "  {}Successfully indexed{} 2 {}documents",
        "".green().bold(),
        "".yellow().bold(),
        "".green().bold()
    );
    println!();

    // Searching section
    println!("{}", "SEARCHING DOCUMENTS".yellow().bold());
    println!("Search through indexed documents:");
    println!(
        "  {} seroost search \"programming language\"",
        "$".bright_black()
    );
    println!();
    println!("{}", "Expected output:".bright_blue());
    println!("  {}Loading search index...{}", "".blue(), "");
    println!(
        "  {}Search results for:{} programming language",
        "".green().bold(),
        "".white().on_blue().bold()
    );
    println!("  {}", "═".repeat(60));
    println!(
        "  {}1.{} ~/documents/samples/{}rust.txt{} (Score: 0.28768)",
        "".yellow().bold(),
        "",
        "".green().bold(),
        ""
    );
    println!(
        "  {}2.{} ~/documents/samples/{}python.txt{} (Score: 0.14384)",
        "".yellow().bold(),
        "",
        "".green().bold(),
        ""
    );
    println!("  {}", "═".repeat(60));
    println!();

    // Subsequent searches
    println!("{}", "SUBSEQUENT SEARCHES".yellow().bold());
    println!("After the first index, you can search without specifying the path again:");
    println!("  {} seroost search \"readability\"", "$".bright_black());
    println!();

    println!("{}", "═".repeat(80).cyan());
    Ok(())
}
