use crate::{packet::ClonablePacket as Packet, parsers::{ParsedData, Protocol, ProtocolParser}};

#[derive(Clone, Copy)]
pub struct CoapParser;

impl ProtocolParser for CoapParser {
    fn parse(&self, packet: &Packet) -> ParsedData {
        // Implement proper COAP parsing logic
        ParsedData{
            protocol: Protocol::Coap(String::from_utf8_lossy(&packet.raw).to_string())}
    }
}
