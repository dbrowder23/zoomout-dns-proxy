use base64::{decode, encode};
use std::fs::{OpenOptions, read_to_string};
use std::io::{self, Write};
use std::path::Path;

/// Path to the blacklist file
const BLACKLIST_FILE: &str = "blacklist.txt";

/// Load and decode blacklist entries from the file
pub fn load_blacklist() -> Vec<String> {
    if !Path::new(BLACKLIST_FILE).exists() {
        println!("Blacklist file not found, creating a new one...");
        if let Err(e) = std::fs::File::create(BLACKLIST_FILE) {
            eprintln!("Failed to create blacklist file: {}", e);
        }
    }

    let content = read_to_string(BLACKLIST_FILE)
        .unwrap_or_else(|_| "".to_string());

    content
        .lines()
        .filter_map(|line| {
            if line.trim().is_empty() {
                None
            } else {
                decode(line.trim())
                    .ok()
                    .and_then(|bytes| String::from_utf8(bytes).ok())
                    .map(|decoded| decoded.to_lowercase())
            }
        })
        .collect()
}

/// Encode and add a new domain to the blacklist
pub fn add_to_blacklist(domain: &str) -> io::Result<()> {
    let encoded = encode(domain.to_lowercase());

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(BLACKLIST_FILE)?;

    writeln!(file, "{}", encoded)?;
    Ok(())
}
