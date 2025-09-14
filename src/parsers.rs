use lopdf::Document;
use select::document::Document as HtmlDocument;
use select::predicate::{Name, Predicate, Text};
use std::error::Error;
use std::fs; // Get the file system.
use std::io;
use std::io::{BufReader, Read}; // Get the io module.
use std::path::Path;
use std::process::exit; // get the process's exit function.
use xml::reader::{EventReader, XmlEvent};

pub type GlobalError = Box<dyn Error>;

pub fn read_entire_pdf_file<P: AsRef<Path>>(file_path: P) -> Result<String, GlobalError> {
    // Load pdf-file
    let doc = Document::load(file_path)?;
    // get the number of pages.
    let pages = doc.get_pages().len();
    // set a const max_page number for ease of processing.
    const MAX_PAGES: usize = 450;
    // create a string for the page content;
    let mut page_content: String = String::new();
    if pages < MAX_PAGES {
        // move page by page.
        for page in 1..=pages {
            // Build an all lower-case string from the text.
            page_content.push_str(
                &*doc.extract_text(&[page as u32])?. // Note that &* here converts String to &str
            chars()
            .map(|character| {
                if character.is_alphabetic(){
                    return character.to_ascii_lowercase();
                }
                character
            }).collect::<String>(),
            );
        }
    } else {
        for page in 1..=MAX_PAGES {
            // Build an all lower-case string from the text.
            page_content.push_str(
                &*doc.extract_text(&[page as u32])?. // Note that &* here converts String to &str
            chars()
            .map(|character| {
                if character.is_alphabetic(){
                    return character.to_ascii_lowercase();
                }
                character
            }).collect::<String>(),
            );
        }
    }

    // Return the file content.
    Ok(page_content)
}

pub fn read_entire_txt_file<P: AsRef<Path>>(file_path: P) -> Result<String, GlobalError> {
    let file = fs::File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let contents = contents
        .chars()
        .map(|character| {
            if character.is_alphabetic() {
                return character.to_ascii_lowercase();
            }
            character
        })
        .collect::<String>();

    Ok(contents)
}

pub fn read_entire_xml_file<P: AsRef<Path>>(file_path: P) -> Result<String, GlobalError> {
    let file = fs::File::open(file_path)?;

    // Create an XML event reader.
    let er = EventReader::new(file);
    // Create a buffer.
    let mut content = String::new();
    // Create an Iterator and map it.
    for event in er.into_iter() {
        if let XmlEvent::Characters(text) = event.unwrap_or_else(|err| {
            // Report the error.
            eprintln!("ERROR: {err}");
            exit(1); // Exit the process with an error code of 1
        }) {
            // Convert to lowercase and add to the content.
            content.push_str(&text.to_ascii_lowercase());
            content.push_str(" "); // For Padding.
        }
    }

    // Return the file content.
    Ok(content)
}

pub fn read_entire_html_file<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let html_content = std::fs::read_to_string(path)?;

    // Parse the HTML
    let document = HtmlDocument::from(html_content.as_str());

    // Extract meaningful text (ignoring scripts, styles, etc.)
    let mut text = String::new();

    // Get content from the body
    for node in document.find(Name("body").descendant(Text)) {
        text.push_str(&node.text());
        text.push(' ');
    }

    Ok(text)
}



pub fn read_code_file<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let code_content = std::fs::read_to_string(&path)?;
    
    // Add line numbers to each line for better search results
    let mut numbered_content = String::new();
    
    for (line_number, line) in code_content.lines().enumerate() {
        // Format: "Line X: content"
        numbered_content.push_str(&format!("Line {}: {}\n", line_number + 1, line));
    }
    
    Ok(numbered_content)
}

// function specifically for getting line information
pub fn get_code_line_info<P: AsRef<Path>>(path: P, search_term: &str) -> Result<Vec<(usize, String)>, io::Error> {
    let code_content = std::fs::read_to_string(&path)?;
    let mut matches = Vec::new();
    
    for (line_number, line) in code_content.lines().enumerate() {
        if line.to_lowercase().contains(&search_term.to_lowercase()) {
            matches.push((line_number + 1, line.to_string()));
        }
    }
    
    Ok(matches)
}


