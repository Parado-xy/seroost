use clap::{Parser, Subcommand};
use colored::*; // Add this import
use serde_json;
use std::collections::HashMap;
use std::fs; // Get the file system.
use std::io;
use std::process;

// Import Modules.
mod lexer;
mod parsers;
mod interact;
mod interactives; 
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
    /// Pass a max file size.
    /// Defaults to 25mb
    #[arg(short, long, default_value = "25")]
    max_file_size: u64,

    #[command(subcommand)]
    command: Option<AppCommands>,
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
    // Create Index Path Variable to  be filled dynamically. 
    let index_path: String;
    // TODO: Add max_file_size to the config file.
    let max_file_size: u64 = cli.max_file_size * 1024u64 * 1024u64;

    // See if an index path was provided.
    if let Some(path) = &cli.index_path {
        let config_path = interact::get_config_path(); // Use the same config path regardless

        // Create parent directories if needed
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Load existing config or create new one
        let mut configuration = if config_path.exists() {
            let file = fs::File::open(&config_path)?;
            // Return the parsed Json File as a HashMap
            serde_json::from_reader(file).unwrap_or_else(|_| HashMap::<String, String>::new())
        } else {
            HashMap::<String, String>::new()
        };

        // Update configuration
        configuration.insert("index_path".to_string(), path.clone());

        // Write back to file
        let file = fs::File::create(&config_path)?;
        serde_json::to_writer(file, &configuration)?;

        // Use in program memory
        index_path = path.to_string();
    } else {
        // Use the SAME config path for reading
        let config_path = interact::get_config_path();

        if !config_path.exists() {
            eprintln!("{}", format!("Error: No index path provided and no saved configuration found.").color("red"));
            eprintln!("{}", format!("Please run the program with --index-path option first:").color("red"));
            eprintln!("{}", format!("    seroost  --index-path /path/to/documents index").color("green"));
            process::exit(0);
        }
        let file = fs::File::open(config_path)?;
        let reader = std::io::BufReader::new(file);

        // Read the JSON contents of the file.
        let config: HashMap<String, String> =
            serde_json::from_reader(reader).expect("Error Parsing The Json configuration file");
        index_path = config
            .get("index_path")
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No index_path in config file"))?
            .clone();
    }

    match &cli.command {
        Some(AppCommands::Index) => {
            interactives::process_file(index_path, max_file_size);
        }
        Some(AppCommands::Search { term }) => {
            interact::search_documents(term)?;
        }
        Some(AppCommands::Usage) => {
            interact::display_usage()?;
        }
        None => {
            println!("{}. Use --help for usage information.", "No command provided".red());
            println!("Or try: {} for detailed examples", "seroost usage".green());
        }
    }

    Ok(())
}





