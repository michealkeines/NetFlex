use crate::{monitors::Packet, parsers::{ParsedData, Protocol, ProtocolParser}};

#[derive(Clone, Copy)]
pub struct CoapParser;

impl ProtocolParser for CoapParser {
    fn parse(&self, packet: &Packet) -> ParsedData {
        // Implement proper COAP parsing logic
        ParsedData{
            protocol: Protocol::Coap(String::from_utf8_lossy(&packet.0).to_string())}
    }
}
