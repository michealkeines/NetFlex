use crate::{monitors::Packet, parsers::{ParsedData, Protocol, ProtocolParser}};

#[derive(Clone, Copy)]
pub struct HttpParser;

impl ProtocolParser for HttpParser {
    fn parse(&self, packet: &Packet) -> ParsedData {
        // Implement proper HTTP parsing logic
        ParsedData{
            protocol: Protocol::Http(String::from_utf8_lossy(&packet.0).to_string())}
    }
}
