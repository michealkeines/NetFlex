use crate::{packet::ClonablePacket as Packet, parsers::{ParsedData, Protocol, ProtocolParser}};


#[derive(Clone, Copy)]
pub struct ArpParser;

impl ProtocolParser for ArpParser {
    fn parse(&self, packet: &Packet) -> ParsedData {
        // Implement proper ARP parsing logic
        ParsedData{
            protocol: Protocol::Arp(String::from_utf8_lossy(&packet.raw).to_string())}
    }
}
