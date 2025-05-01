use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::SystemTime;
use uuid::Uuid;

use crate::extractor::PacketInfo;

// --- Asset struct and methods ---
#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    pub asset_name: String,
    pub src_mac: String,
    pub src_ip: Option<String>,
    pub first_detected: SystemTime,
    pub last_detected: SystemTime,
    pub packet_history: Vec<PacketInfo>,
    pub external_ports_connected_to: HashSet<u16>,
    pub protocols_used: HashSet<String>,
}

impl Asset {
    /// Load existing asset store from a JSON file
    pub fn load_from_file(filename: &str) -> HashMap<String, Asset> {
        let file = std::fs::File::open(filename);
        match file {
            Ok(f) => {
                let reader = std::io::BufReader::new(f);
                serde_json::from_reader(reader).unwrap_or_else(|_| HashMap::new())
            }
            Err(_) => HashMap::new(), // file doesn't exist, return empty map
        }
    }

    /// Write updated asset store to file
    pub fn dump_to_file(assets: &HashMap<String, Asset>, filename: &str) {
        let file = std::fs::File::create(filename).expect("Could not create output file");
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, assets).expect("Serialization failed");
    }

    /// Add or update an asset based on a single packet
    pub fn analyze_single(
        packet: &PacketInfo,
        mut existing_assets: HashMap<String, Asset>,
    ) -> HashMap<String, Asset> {
        let src_mac = match &packet.src_mac {
            Some(mac) => mac.clone(),
            None => return existing_assets, // MAC is mandatory
        };

        let asset_key = match &packet.src_ip {
            Some(ip) => format!("{}-{}", src_mac, ip),
            None => src_mac.clone(),
        };

        let now = SystemTime::now();
        let asset = existing_assets
            .entry(asset_key.clone())
            .or_insert_with(|| Asset {
                asset_name: format!("asset-{}", Uuid::new_v4()),
                src_mac: src_mac.clone(),
                src_ip: packet.src_ip.clone(),
                first_detected: now,
                last_detected: now,
                packet_history: Vec::new(),
                external_ports_connected_to: HashSet::new(),
                protocols_used: HashSet::new(),
            });

        asset.last_detected = now;
        asset.packet_history.push(packet.clone());

        if let Some(proto) = &packet.protocol_info {
            asset.protocols_used.insert(proto.protocol_name.clone());
        }

        if let Some(dst_port) = packet.dst_port {
            asset.external_ports_connected_to.insert(dst_port);
        }

        existing_assets
    }
}
