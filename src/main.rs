mod config;
mod dns_proxy;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let routes = load_routes("routes.txt")?;
    let proxy = dns_proxy::DnsProxy::new("127.0.0.1:53", "blacklist.txt", routes).await?;
    proxy.run().await
}

fn load_routes<P: AsRef<Path>>(path: P) -> io::Result<HashMap<String, String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut routes = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() == 2 {
            routes.insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    Ok(routes)
}
