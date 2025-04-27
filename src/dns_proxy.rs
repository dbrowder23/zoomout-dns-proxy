use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use trust_dns_proto::op::{Message, MessageType, OpCode};
use trust_dns_proto::rr::{Name, RData, Record, RecordType};
use trust_dns_proto::serialize::binary::{BinDecodable, BinEncodable};

use crate::config::{append_to_blacklist, load_blacklist};

pub struct DnsProxy {
    socket: Arc<UdpSocket>,
    blacklist: Vec<String>,
}

impl DnsProxy {
    pub async fn new(bind_addr: &str, blacklist_path: &str) -> std::io::Result<Self> {
        let socket = Arc::new(UdpSocket::bind(bind_addr).await?);
        let blacklist = load_blacklist(blacklist_path)?;
        Ok(DnsProxy { socket, blacklist })
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let mut buf = [0u8; 512];

        loop {
            let (len, addr) = self.socket.recv_from(&mut buf).await?;
            let data = &buf[..len];

            if let Ok(request) = Message::from_bytes(data) {
                if let Some(query) = request.queries().first() {
                    let domain = query.name().to_utf8();

                    if self.blacklist.contains(&domain) {
                        let mut response = Message::new();
                        response.set_id(request.id());
                        response.set_message_type(MessageType::Response);
                        response.set_op_code(OpCode::Query);
                        response.set_authoritative(true);
                        response.add_query(query.clone());

                        let record = Record::from_rdata(
                            query.name().clone(),
                            300,
                            RData::A("127.0.0.1".parse().unwrap()),
                        );
                        response.add_answer(record);

                        let response_bytes = response.to_bytes()?;
                        self.socket.send_to(&response_bytes, &addr).await?;
                        continue;
                    }
                }
            }

            // Forward the request to an upstream DNS server (e.g., 8.8.8.8)
            let upstream_addr: SocketAddr = "8.8.8.8:53".parse().unwrap();
            let upstream_socket = UdpSocket::bind("0.0.0.0:0").await?;
            upstream_socket.send_to(data, &upstream_addr).await?;

            let mut upstream_buf = [0u8; 512];
            let (up_len, _) = upstream_socket.recv_from(&mut upstream_buf).await?;
            self.socket.send_to(&upstream_buf[..up_len], &addr).await?;
        }
    }
}
