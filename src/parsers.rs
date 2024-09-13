use async_trait::async_trait;
use crate::packet::ClonablePacket as Packet;
use crate::protocols::{arp::ArpParser, dns::DnsParser, http::HttpParser, coap::CoapParser, raw::RawParser};

#[async_trait]
// Define a trait for common parsing behavior
pub trait ProtocolParser {
    fn parse(&self, packet: &Packet) -> ParsedData;
}

#[derive(Debug)]
pub struct ParsedData {
    pub protocol: Protocol,
}

// Define the Protocol enum with support for different protocols
#[derive(Debug)]
pub enum Protocol {
    Http(String),
    Dns(String),
    Arp(String),
    Coap(String),
    Raw(Vec<u8>),
}

// Create a parser registry or factory
pub struct ParserRegistry {
    http_parser: HttpParser,
    dns_parser: DnsParser,
    arp_parser: ArpParser,
    coap_parser: CoapParser,
    raw_parser: RawParser,
}

impl ParserRegistry {
    pub fn new() -> Self {
        ParserRegistry {
            http_parser: HttpParser,
            dns_parser: DnsParser,
            arp_parser: ArpParser,
            coap_parser: CoapParser,
            raw_parser: RawParser,
        }
    }

    pub fn get_parser(&self, data: &[u8]) -> Box<dyn ProtocolParser> {
        //println!("{data:?}");
        if data.starts_with(b"GET") {
            Box::new(self.http_parser)
        } else if data.starts_with(b"\x00\x01") {
            Box::new(self.dns_parser)
        } else if data.starts_with(b"\x00\x01\x08\x00") {
            Box::new(self.arp_parser)
        } else if data.starts_with(b"\x11\x00") {
            Box::new(self.coap_parser)
        } else {
            Box::new(self.raw_parser)
        }
    }
}

