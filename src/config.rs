use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use base64::{engine::general_purpose, Engine as _};

pub fn load_blacklist<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut domains = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let decoded = general_purpose::STANDARD
            .decode(line.trim())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let domain = String::from_utf8(decoded)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        domains.push(domain);
    }

    Ok(domains)
}

pub fn append_to_blacklist<P: AsRef<Path>>(path: P, domain: &str) -> io::Result<()> {
    let encoded = general_purpose::STANDARD.encode(domain.to_lowercase());
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{}", encoded)?;
    Ok(())
}
