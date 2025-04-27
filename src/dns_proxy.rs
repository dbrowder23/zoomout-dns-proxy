use trust_dns_proto::op::{Message, MessageType, OpCode, Query};
use trust_dns_proto::rr::{RecordType, RecordClass, Name, RData, Record};
use trust_dns_proto::rr::rdata::A;
use trust_dns_proto::serialize::binary::*;
use tokio::net::UdpSocket;
use std::net::SocketAddr;
use regex::Regex;
use crate::config;

pub struct DnsProxy {
    listen_addr: String,
    upstream_addr: String,
    blacklist: Vec<String>,
}

impl DnsProxy {
    pub fn new(listen_addr: &str, upstream_addr: String, blacklist: Vec<String>) -> Self {
        Self {
            listen_addr: listen_addr.to_string(),
            upstream_addr,
            blacklist,
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let socket = UdpSocket::bind(&self.listen_addr).await?;
        println!("Listening for DNS requests on {}", &self.listen_addr);

        let mut buf = [0u8; 512];

        loop {
            let (len, addr) = socket.recv_from(&mut buf).await?;
            let request = buf[..len].to_vec();
            let blacklist = self.blacklist.clone();
            let upstream = self.upstream_addr.clone();
            let socket = socket.try_clone()?;

            tokio::spawn(async move {
                if let Err(e) = Self::handle_request(socket, addr, request, blacklist, upstream).await {
                    eprintln!("Error handling DNS request: {:?}", e);
                }
            });
        }
    }

    async fn handle_request(
        socket: UdpSocket,
        addr: SocketAddr,
        request: Vec<u8>,
        blacklist: Vec<String>,
        upstream: String,
    ) -> anyhow::Result<()> {
        let mut decoder = BinDecoder::new(&request);
        let message = Message::read(&mut decoder)?;

        let queries = message.queries();
        if queries.is_empty() {
            return Ok(());
        }

        let query_name = queries[0].name().to_utf8().to_lowercase();

        let is_blacklisted = blacklist.iter().any(|pattern| query_name.contains(pattern));

        let suspicious_but_new = query_name.ends_with(".zoom.us")
            && (query_name.contains("api") || query_name.contains("track"));

        let mut response = Message::new();
        response.set_id(message.id());
        response.set_message_type(MessageType::Response);
        response.set_op_code(OpCode::Query);
        response.set_authoritative(true);

        if is_blacklisted {
            println!("Blocked blacklisted domain: {}", query_name);

            let record = Record::from_rdata(
                queries[0].name().clone(),
                60,
                RData::A(A::new(127, 0, 0, 1)),
            );

            response.add_query(queries[0].clone());
            response.add_answer(record);
        } else if suspicious_but_new {
            println!("Suspicious new domain detected and added: {}", query_name);
            let _ = config::add_to_blacklist(&query_name);

            let record = Record::from_rdata(
                queries[0].name().clone(),
                60,
                RData::A(A::new(127, 0, 0, 1)),
            );

            response.add_query(queries[0].clone());
            response.add_answer(record);
        } else {
            // Forward to upstream
            println!("Allowed domain: {}", query_name);

            let forward_socket = UdpSocket::bind("0.0.0.0:0").await?;
            forward_socket.send_to(&request, &upstream).await?;

            let mut upstream_buf = [0u8; 512];
            let (len, _) = forward_socket.recv_from(&mut upstream_buf).await?;

            socket.send_to(&upstream_buf[..len], addr).await?;
            return Ok(());
        }

        let mut response_bytes = Vec::with_capacity(512);
        let mut encoder = BinEncoder::new(&mut response_bytes);
        response.emit(&mut encoder)?;

        socket.send_to(&response_bytes, addr).await?;

        Ok(())
    }
}
