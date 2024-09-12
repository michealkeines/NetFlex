use crate::{monitors::Packet, parsers::{ParsedData, Protocol, ProtocolParser}};

#[derive(Clone, Copy)]
pub struct DnsParser;

impl ProtocolParser for DnsParser {
    fn parse(&self, packet: &Packet) -> ParsedData {
        // Implement proper DNS parsing logic
        ParsedData{
            protocol: Protocol::Dns(String::from_utf8_lossy(&packet.0).to_string())}
    }
}
