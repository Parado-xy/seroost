#![allow(dead_code)]

// Bring modules into scope. 
use colored::Colorize; 
use crate::parsers; 
use std::fs;
use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::io; 

// Define 2 types.
type TermFreq = HashMap<String, usize>;
type TermFreqIndex = HashMap<PathBuf, TermFreq>;

// Bring native crates.
use crate::lexer; 

pub fn display_usage() -> Result<(), parsers::GlobalError> {
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


pub fn process_files(
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
                    process_files(
                        path.to_str().unwrap_or(dir_path),
                        term_frequency_index,
                        max_file_size,
                    )?;
                    continue;
                }
                // Print this message to inform the user of a skipped large file. 
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

                // Process files based on extensions. 
                let content: Vec<char>;
                match path.extension() {
                    Some(ext) => match ext.to_string_lossy().to_lowercase().as_str() {
                        "pdf" => match parsers::read_entire_pdf_file(&path) {
                            Ok(text) => content = text.chars().collect::<Vec<_>>(),
                            Err(e) => {
                                eprintln!(
                                    "{} {:?}: {}",
                                    "Error processing PDF file:".red(),
                                    path,
                                    e
                                );
                                continue;
                            }
                        },
                        "txt" => match parsers::read_entire_txt_file(&path) {
                            Ok(text) => content = text.chars().collect::<Vec<_>>(),
                            Err(e) => {
                                eprintln!(
                                    "{} {:?}: {}",
                                    "Error processing text file:".red(),
                                    path,
                                    e
                                );
                                continue;
                            }
                        },
                        "xml" | "xhtml" => match parsers::read_entire_xml_file(&path) {
                            Ok(text) => content = text.chars().collect::<Vec<_>>(),
                            Err(e) => {
                                eprintln!(
                                    "{} {:?}: {}",
                                    "Error processing XML file:".red(),
                                    path,
                                    e
                                );
                                continue;
                            }
                        },
                        "html" | "htm" => match parsers::read_entire_html_file(&path) {
                            Ok(text) => content = text.chars().collect::<Vec<_>>(),
                            Err(e) => {
                                eprintln!(
                                    "{} {:?}: {}",
                                    "Error processing HTML file:".red(),
                                    path,
                                    e
                                );
                                continue;
                            }
                        },
                        _ => {
                            eprintln!("Error: do not know how to process file: {path:?}. Skipping file...");
                            continue;
                        }
                    },
                    None => {
                        eprintln!(
                            "{}: do not know how to process this file, couldn't discern the extension: {path:?}
                             Skipping file...",
                            "Error".red()
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

pub fn index_documents(dir_path: &str, max_file_size: u64) -> Result<(), parsers::GlobalError> {
    // Create a new empty index first
    let mut term_frequency_index = TermFreqIndex::new();

    // Process documents and populate the index
    println!(
        "{} {}",
        "Indexing directory:".green().bold(),
        dir_path.blue()
    );
    process_files(dir_path, &mut term_frequency_index, max_file_size)?;

    // Save the complete index only once after all processing is done
    let index_path :PathBuf = get_indeces_path();
    if let Some(parent) = Path::new(&index_path).parent() {
        fs::create_dir_all(parent)?;
    }
    println!("{} {}", "Saving index to:".green(), index_path.to_str().expect("Invalid Path Name").blue());
    let index_file = fs::File::create(index_path)?;
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

pub fn search_documents(query: &str) -> Result<(), parsers::GlobalError> {
    // Load the index
    let index_path = get_indeces_path();
    if !Path::new(&index_path).exists() {
        eprintln!(
            "{}",
            "Error: index file not found. Please run index first."
                .red()
                .bold()
        );
        return Ok(());
    }

    let index_file = fs::File::open(&index_path)?;
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



/// Returns the configuration path based on the system used.
/// If no config path found, it results to directory based config storage.
pub fn get_config_path() -> PathBuf {
    match dirs::config_dir() {
        Some(path) => path.join("seroost").join("config.json"),
        None => PathBuf::from("./indeces/config.json"),
    }
}

/// Returns the configuration path based on the system used.
/// If no config path found, it results to directory based index storage.
pub fn get_indeces_path() -> PathBuf {
    match dirs::config_dir(){
        Some(path) => path.join("seroost").join("index.json"),
        None => PathBuf::from("./indeces/index.json"),        
    }
}
