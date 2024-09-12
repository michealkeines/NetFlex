use crate::{monitors::Packet, parsers::{ParsedData, Protocol, ProtocolParser}};

#[derive(Clone, Copy)]
// Implement a RawParser for handling raw data
pub struct RawParser;

impl ProtocolParser for RawParser {
    fn parse(&self, packet: &Packet) -> ParsedData {
        ParsedData {
            protocol: Protocol::Raw(packet.0.clone()),
        }
    }
}