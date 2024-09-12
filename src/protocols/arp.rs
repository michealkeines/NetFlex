use crate::{monitors::Packet, parsers::{ParsedData, Protocol, ProtocolParser}};


#[derive(Clone, Copy)]
pub struct ArpParser;

impl ProtocolParser for ArpParser {
    fn parse(&self, packet: &Packet) -> ParsedData {
        // Implement proper ARP parsing logic
        ParsedData{
            protocol: Protocol::Arp(String::from_utf8_lossy(&packet.0).to_string())}
    }
}
