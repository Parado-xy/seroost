// Bring modules into scope.
use crate::parsers;
use colored::Colorize;
use crossbeam::channel::{self, unbounded};
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::thread::{self, JoinHandle};
use std::fs;

// Define 2 types.
type TermFreq = HashMap<String, usize>;
type TermFreqIndex = HashMap<PathBuf, TermFreq>;

// Bring native crates.
use crate::lexer;
use crate::interact; 

pub fn traverse_dirs<P: AsRef<Path>>(dir_path: P, sender: channel::Sender<String>) {
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            // Handler directory recursion.
            if path.is_dir() {
                // Pass a clone of the sender because it's mpsc;
                traverse_dirs(path, sender.clone());
            } else {
                if path.is_file() {
                    // Send the path;
                    sender.send(path.to_string_lossy().to_string()).unwrap();
                }
            }
        }
    }
}

pub fn process_file(path: String, max_file_size: u64) {
    // Create sender and reciever channels for directory traversal.
    let (file_sender, file_reciever) = unbounded::<String>();

    // Create sender and reciever channels for file processing threads.
    let (processing_sender, processing_reciever) = unbounded::<(String, Vec<char>)>();

    // Directory traversal should happen on its onw thread.
    let file_sender_clone = file_sender.clone(); // clone the file_sender outside of the closure so we don't move it. 
    let dir_traversal_handle: JoinHandle<()> = thread::spawn(move || {
        traverse_dirs(path.to_owned(), file_sender_clone);
    });

    // TermFrequency Calculation Should happen on its own thread. It returns the term frequency index table.
    let processing_reciever_clone = processing_reciever.clone();
    let term_frequency_calc_handle =
        thread::spawn(|| calculate_term_frequency(processing_reciever_clone));
   

    // Create a pool of worker threads.
    let mut handles: Vec<JoinHandle<()>> = vec![];

    let num_threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(2);
    for _ in 0..num_threads {
        // CLone file reciever.
        let file_reciever = file_reciever.clone();
        // Clone file processing sender.
        let processing_sender = processing_sender.clone();
        // Spawn a thread.
        let handle = thread::spawn(move || {
            while let Ok(file_path) = file_reciever.recv() {
                // Print this message to inform the user of a skipped large file.
                if let Ok(metadata) = fs::metadata(&file_path) {
                    let file_size = metadata.len();
                    if file_size > max_file_size {
                        println!(
                            "{} {:?} ({:.2}MB)",
                            "Skipping large file:".yellow(),
                            file_path,
                            file_size as f64 / (1024.0 * 1024.0)
                        );
                        continue;
                    }
                }

                // Process files based on extensions.
                match Path::new(&file_path).extension() {
                    Some(ext) => match ext.to_string_lossy().to_lowercase().as_str() {
                        "pdf" => match parsers::read_entire_pdf_file(&file_path) {
                            Ok(text) => {
                                // send the processed text and file_path
                                let content = text.chars().collect::<Vec<_>>();
                                let _ =
                                    processing_sender.send((file_path, content)).map_err(|err| {
                                        eprintln!(
                                            "{} : {}",
                                            "Error sending file content to recieving channel:"
                                                .red(),
                                            err
                                        );
                                    });
                            }
                            Err(e) => {
                                eprintln!(
                                    "{} {:?}: {}",
                                    "Error processing PDF file:".red(),
                                    file_path,
                                    e
                                );
                                continue;
                            }
                        },
                        "txt" => match parsers::read_entire_txt_file(&file_path) {
                            Ok(text) => {
                                // send the processed text and file_path
                                let content = text.chars().collect::<Vec<_>>();
                                let _ =
                                    processing_sender.send((file_path, content)).map_err(|err| {
                                        eprintln!(
                                            "{} : {}",
                                            "Error sending file content to recieving channel:"
                                                .red(),
                                            err
                                        );
                                    });
                            }
                            Err(e) => {
                                eprintln!(
                                    "{} {:?}: {}",
                                    "Error processing text file:".red(),
                                    file_path,
                                    e
                                );
                                continue;
                            }
                        },
                        "xml" | "xhtml" => match parsers::read_entire_xml_file(&file_path) {
                            Ok(text) => {
                                // send the processed text and file_path
                                let content = text.chars().collect::<Vec<_>>();
                                let _ =
                                    processing_sender.send((file_path, content)).map_err(|err| {
                                        eprintln!(
                                            "{} : {}",
                                            "Error sending file content to recieving channel:"
                                                .red(),
                                            err
                                        );
                                    });
                            }
                            Err(e) => {
                                eprintln!(
                                    "{} {:?}: {}",
                                    "Error processing XML file:".red(),
                                    file_path,
                                    e
                                );
                                continue;
                            }
                        },
                        "html" | "htm" => match parsers::read_entire_html_file(&file_path) {
                            Ok(text) => {
                                // send the processed text and file_path
                                let content = text.chars().collect::<Vec<_>>();
                                let _ =
                                    processing_sender.send((file_path, content)).map_err(|err| {
                                        eprintln!(
                                            "{} : {}",
                                            "Error sending file content to recieving channel:"
                                                .red(),
                                            err
                                        );
                                    });
                            }
                            Err(e) => {
                                eprintln!(
                                    "{} {:?}: {}",
                                    "Error processing HTML file:".red(),
                                    file_path,
                                    e
                                );
                                continue;
                            }
                        },
                        _ => {
                            eprintln!(
                                "{}: do not know how to process this file, couldn't discern the extension: {file_path:?}
                                Skipping file...",
                                "Error".red()
                            );
                        }
                    },
                    None => {
                        eprintln!(
                            "{}: do not know how to process this file, couldn't discern the extension: {file_path:?}
                             Skipping file...",
                            "Error".red()
                        );
                        continue;
                    }
                }
            }
        });

        // Add each handle to the array.
        handles.push(handle);
    }

    // After directory traversal completes
    let _ = dir_traversal_handle.join();

    // Drop the original sender to signal no more files will be sent
    drop(file_sender);

    // Wait for all worker threads to complete
    for handle in handles {
        let _ = handle.join();
    }

    // Now drop the processing sender to signal no more files to process
    drop(processing_sender);

    // Now...
    if let Ok(term_freq_index) = term_frequency_calc_handle.join() {
        // Save the complete index only once after all processing is done
        let index_path: PathBuf = interact::get_indeces_path();
        if let Some(parent) = Path::new(&index_path).parent() {
            let _ = fs::create_dir_all(parent).map_err(|err|{
                eprintln!(
                    "{} {err}",
                    "Error creating parent directory for index Path:".red(),   
                )
            });
        }
        println!(
            "{} {}",
            "Saving index to:".green(),
            index_path.to_str().expect("Invalid Path Name").blue()
        );
        if let Ok(index_file) = fs::File::create(index_path){
            let  _ = serde_json::to_writer(index_file, &term_freq_index)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()));
        }    
        println!(
            "{} {} {}",
            "Successfully indexed".green().bold(),
            term_freq_index.len().to_string().yellow().bold(),
            "documents".green().bold()
        );
    }
}

fn calculate_term_frequency(
    processing_reciever: channel::Receiver<(String, Vec<char>)>,
) -> TermFreqIndex {
    // Create a new empty index first
    let mut term_frequency_index = TermFreqIndex::new();

    while let Ok((file_path, content)) = processing_reciever.recv() {
        // Print the indexing status of a file.
        println!(
            "{} {file_path}",
            "Indexing: ".green()
        );

        // Instantiate term frequency hashmap.
        let mut term_freq = TermFreq::new();
        // Instantiate Lexer.
        let lexer = lexer::Lexer::new(&content);

        // Iterate through the lexer.
        for token in lexer {
            let term = token;
            // Check if the term already exists in the HashMap/Table.
            if let Some(count) = term_freq.get_mut(&term) {
                *count += 1; // Increment the frequency count of the word in the doc if it already exists.
            } else {
                // `move` the term into the hashmap/table because it gets dropped after this point regardless.
                term_freq.insert(term, 1); // Start The Term Count Up with a 1
            }
        }
        // Add a file and its term frequency to the term_frequeny_index table.
        term_frequency_index.insert(PathBuf::from(file_path.clone()), term_freq);
    }
    // Return the hashmap
    term_frequency_index
}
