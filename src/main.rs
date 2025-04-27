mod config;
mod dns_proxy;
mod utils;

use dns_proxy::DnsProxy;
use utils::init_logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger();

    println!("ZoomOut DNS Proxy - Dynamic DNS Interceptor");
    println!("Starting DNS proxy server on 127.0.0.1:53...");

    let blacklist = config::load_blacklist();
    let upstream_dns = "8.8.8.8:53".to_string(); // Google's DNS

    let proxy = DnsProxy::new("127.0.0.1:53", upstream_dns, blacklist);
    proxy.run().await?;

    Ok(())
}
