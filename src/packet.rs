use pnet::packet::{
    arp::ArpPacket, ethernet::{EtherTypes, EthernetPacket}, icmp::IcmpPacket, icmpv6::Icmpv6Packet, 
    ipv4::Ipv4Packet, ipv6::Ipv6Packet, tcp::TcpPacket, udp::UdpPacket, Packet, ip::IpNextHeaderProtocols
};

#[derive(Debug, Clone)]
pub struct ClonableEthernetPacket {
    pub data: Vec<u8>,
}

impl ClonableEthernetPacket {
    pub fn new(data: &[u8]) -> Option<Self> {
        Some(ClonableEthernetPacket {
            data: data.to_vec(),
        })
    }

    pub fn parse(&self) -> Option<EthernetPacket> {
        EthernetPacket::new(&self.data)
    }
}

#[derive(Debug, Clone)]
pub struct ClonableIpv4Packet {
    pub data: Vec<u8>,
}

impl ClonableIpv4Packet {
    pub fn new(data: &[u8]) -> Option<Self> {
        Some(ClonableIpv4Packet {
            data: data.to_vec(),
        })
    }

    pub fn parse(&self) -> Option<Ipv4Packet> {
        Ipv4Packet::new(&self.data)
    }
}

#[derive(Debug, Clone)]
pub struct ClonableIpv6Packet {
    pub data: Vec<u8>,
}

impl ClonableIpv6Packet {
    pub fn new(data: &[u8]) -> Option<Self> {
        Some(ClonableIpv6Packet {
            data: data.to_vec(),
        })
    }

    pub fn parse(&self) -> Option<Ipv6Packet> {
        Ipv6Packet::new(&self.data)
    }
}

#[derive(Debug, Clone)]
pub struct ClonableTcpPacket {
    pub data: Vec<u8>,
}

impl ClonableTcpPacket {
    pub fn new(data: &[u8]) -> Option<Self> {
        Some(ClonableTcpPacket {
            data: data.to_vec(),
        })
    }

    pub fn parse(&self) -> Option<TcpPacket> {
        TcpPacket::new(&self.data)
    }
}

#[derive(Debug, Clone)]
pub struct ClonableUdpPacket {
    pub data: Vec<u8>,
}

impl ClonableUdpPacket {
    pub fn new(data: &[u8]) -> Option<Self> {
        Some(ClonableUdpPacket {
            data: data.to_vec(),
        })
    }

    pub fn parse(&self) -> Option<UdpPacket> {
        UdpPacket::new(&self.data)
    }
}

#[derive(Debug, Clone)]
pub struct ClonableIcmpPacket {
    pub data: Vec<u8>,
}

impl ClonableIcmpPacket {
    pub fn new(data: &[u8]) -> Option<Self> {
        Some(ClonableIcmpPacket {
            data: data.to_vec(),
        })
    }

    pub fn parse(&self) -> Option<IcmpPacket> {
        IcmpPacket::new(&self.data)
    }
}

#[derive(Debug, Clone)]
pub struct ClonableIcmpv6Packet {
    pub data: Vec<u8>,
}

impl ClonableIcmpv6Packet {
    pub fn new(data: &[u8]) -> Option<Self> {
        Some(ClonableIcmpv6Packet {
            data: data.to_vec(),
        })
    }

    pub fn parse(&self) -> Option<Icmpv6Packet> {
        Icmpv6Packet::new(&self.data)
    }
}

#[derive(Debug, Clone)]
pub struct ClonableArpPacket {
    pub data: Vec<u8>,
}

impl ClonableArpPacket {
    pub fn new(data: &[u8]) -> Option<Self> {
        Some(ClonableArpPacket {
            data: data.to_vec(),
        })
    }

    pub fn parse(&self) -> Option<ArpPacket> {
        ArpPacket::new(&self.data)
    }
}

#[derive(Debug, Clone)]
pub struct ClonablePacket {
    pub raw: Vec<u8>,
    pub ethernet: Option<ClonableEthernetPacket>,
    pub ipv4: Option<ClonableIpv4Packet>,
    pub ipv6: Option<ClonableIpv6Packet>,
    pub tcp: Option<ClonableTcpPacket>,
    pub udp: Option<ClonableUdpPacket>,
    pub icmp: Option<ClonableIcmpPacket>,
    pub icmpv6: Option<ClonableIcmpv6Packet>,
    pub arp: Option<ClonableArpPacket>,
}

impl ClonablePacket {
    pub fn new(raw_data: Vec<u8>) -> Self {
        let ethernet = ClonableEthernetPacket::new(&raw_data);
        let mut ipv4 = None;
        let mut ipv6 = None;
        let mut arp = None;
        let mut tcp = None;
        let mut udp = None;
        let mut icmp = None;
        let mut icmpv6 = None;

        if let Some(eth_packet) = ethernet.as_ref().and_then(|e| e.parse()) {
            match eth_packet.get_ethertype() {
                EtherTypes::Ipv4 => {
                    ipv4 = ClonableIpv4Packet::new(eth_packet.payload());
                    if let Some(ipv4_packet) = ipv4.as_ref().and_then(|p| p.parse()) {
                        match ipv4_packet.get_next_level_protocol() {
                            IpNextHeaderProtocols::Tcp => {
                                tcp = ClonableTcpPacket::new(ipv4_packet.payload());
                            }
                            IpNextHeaderProtocols::Udp => {
                                udp = ClonableUdpPacket::new(ipv4_packet.payload());
                            }
                            IpNextHeaderProtocols::Icmp => {
                                icmp = ClonableIcmpPacket::new(ipv4_packet.payload());
                            }
                            _ => {}
                        }
                    }
                }
                EtherTypes::Ipv6 => {
                    ipv6 = ClonableIpv6Packet::new(eth_packet.payload());
                    if let Some(ipv6_packet) = ipv6.as_ref().and_then(|p| p.parse()) {
                        match ipv6_packet.get_next_header() {
                            IpNextHeaderProtocols::Tcp => {
                                tcp = ClonableTcpPacket::new(ipv6_packet.payload());
                            }
                            IpNextHeaderProtocols::Udp => {
                                udp = ClonableUdpPacket::new(ipv6_packet.payload());
                            }
                            IpNextHeaderProtocols::Icmpv6 => {
                                icmpv6 = ClonableIcmpv6Packet::new(ipv6_packet.payload());
                            }
                            _ => {}
                        }
                    }
                }
                EtherTypes::Arp => {
                    arp = ClonableArpPacket::new(eth_packet.payload());
                }
                _ => {}
            }
        }

        ClonablePacket {
            raw: raw_data,
            ethernet,
            ipv4,
            ipv6,
            tcp,
            udp,
            icmp,
            icmpv6,
            arp,
        }
    }
}
