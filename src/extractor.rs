use std::net::Ipv4Addr;
use dashmap::DashSet; // Use DashSet for uniqueness
use pnet::packet::{
    ethernet::EthernetPacket, 
    ipv4::Ipv4Packet, 
    ipv6::Ipv6Packet, 
    tcp::TcpPacket, 
    udp::UdpPacket, 
    arp::ArpPacket, 
    Packet
};
use std::sync::Arc;
use crate::packet::ClonablePacket;
use std::hash::{Hash, Hasher};

// Implement Hash and PartialEq for PacketInfo to ensure uniqueness
#[derive(Debug, Clone, Eq)]
pub struct PacketInfo {
    pub src_mac: Option<String>,
    pub dst_mac: Option<String>,
    pub src_ip: Option<String>,
    pub dst_ip: Option<String>,
    pub src_port: Option<u16>,
    pub dst_port: Option<u16>,
}

impl Hash for PacketInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.src_mac.hash(state);
        self.dst_mac.hash(state);
        self.src_ip.hash(state);
        self.dst_ip.hash(state);
        self.src_port.hash(state);
        self.dst_port.hash(state);
    }
}

impl PartialEq for PacketInfo {
    fn eq(&self, other: &Self) -> bool {
        self.src_mac == other.src_mac &&
        self.dst_mac == other.dst_mac &&
        self.src_ip == other.src_ip &&
        self.dst_ip == other.dst_ip &&
        self.src_port == other.src_port &&
        self.dst_port == other.dst_port
    }
}

pub struct InformationExtractor {
    pub db: Arc<DashSet<PacketInfo>>, // Use DashSet for storing unique PacketInfo
}

impl InformationExtractor {
    pub fn new() -> Self {
        InformationExtractor {
            db: Arc::new(DashSet::new()), // Initialize the DashSet
        }
    }

    pub fn extract_and_store(&self, packet: &ClonablePacket) {
        let mut info = PacketInfo {
            src_mac: None,
            dst_mac: None,
            src_ip: None,
            dst_ip: None,
            src_port: None,
            dst_port: None,
        };

        // Extract MAC addresses
        if let Some(eth_packet) = &packet.ethernet {
            if let Some(eth) = eth_packet.parse() {
                info.src_mac = Some(format!("{}", eth.get_source()));
                info.dst_mac = Some(format!("{}", eth.get_destination()));
            }
        }

        // Extract IP addresses and transport layer ports
        if let Some(ipv4_packet) = &packet.ipv4 {
            if let Some(ipv4) = ipv4_packet.parse() {
                info.src_ip = Some(Ipv4Addr::from(ipv4.get_source()).to_string());
                info.dst_ip = Some(Ipv4Addr::from(ipv4.get_destination()).to_string());
            }
        } else if let Some(ipv6_packet) = &packet.ipv6 {
            if let Some(ipv6) = ipv6_packet.parse() {
                info.src_ip = Some(format!("{}", ipv6.get_source()));
                info.dst_ip = Some(format!("{}", ipv6.get_destination()));
            }
        }

        // Extract transport layer ports
        if let Some(tcp_packet) = &packet.tcp {
            if let Some(tcp) = tcp_packet.parse() {
                info.src_port = Some(tcp.get_source());
                info.dst_port = Some(tcp.get_destination());
            }
        } else if let Some(udp_packet) = &packet.udp {
            if let Some(udp) = udp_packet.parse() {
                info.src_port = Some(udp.get_source());
                info.dst_port = Some(udp.get_destination());
            }
        }

        // Extract ARP information
        if let Some(arp_packet) = &packet.arp {
            if let Some(arp) = arp_packet.parse() {
                info.src_mac = Some(format!("{}", arp.get_sender_hw_addr()));
                info.dst_mac = Some(format!("{}", arp.get_target_hw_addr()));
                info.src_ip = Some(Ipv4Addr::from(arp.get_sender_proto_addr()).to_string());
                info.dst_ip = Some(Ipv4Addr::from(arp.get_target_proto_addr()).to_string());
            }
        }
       // println!("info: {info:?}");
        // Store extracted information in the DashSet (only unique entries will be stored)
        self.db.insert(info);
    }

    // Method to retrieve all the unique packet information
    pub fn get_all_packet_info(&self) -> Vec<PacketInfo> {
        self.db.iter().map(|p| p.clone()).collect()
    }
}

