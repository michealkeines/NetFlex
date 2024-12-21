use std::sync::{Arc, Mutex};
use pcap::Capture;
use async_trait::async_trait;
use tokio::task;
use crate::packet::ClonablePacket as Packet;

#[async_trait]
pub trait TrafficMonitor {
    async fn capture_traffic(&self) -> Packet;
}

pub struct InterfaceMonitor {
    pub device_name: String,
}

#[async_trait]
impl TrafficMonitor for InterfaceMonitor {
    async fn capture_traffic(&self) -> Packet {
        task::spawn_blocking({
            let device_name = self.device_name.clone();
            move || {
                let mut cap = Capture::from_device(&device_name[..]).unwrap()
                    .immediate_mode(true)
                    .open().unwrap();
                let packet = cap.next_packet().unwrap();
                // Parsing the raw packet
                Packet::new(packet.data.to_vec())
            }
        }).await.unwrap()
    }
}

pub struct FileMonitor {
    pub cap: Arc<Mutex<Capture<pcap::Offline>>>, // Store the capture session in a thread-safe Arc<Mutex<>>
}

impl FileMonitor {
    pub fn new(file_path: String) -> Self {
        let cap = Capture::from_file(&file_path).expect("Failed to open PCAP file");
        FileMonitor {
            cap: Arc::new(Mutex::new(cap)),
        }
    }
}

#[async_trait]
impl TrafficMonitor for FileMonitor {
    async fn capture_traffic(&self) -> Packet {
        let cap = Arc::clone(&self.cap); // Clone the Arc for thread-safe access
        task::spawn_blocking(move || {
            let mut cap = cap.lock().expect("Failed to lock PCAP capture");
            let next = cap.next_packet();
            if let Ok(packet) = next {
                Packet::new(packet.data.to_vec());
            }
           // println!("packet: {:?}", packet);
            // Parsing the raw packet
            Packet::new(vec![])
        }).await.unwrap()
    }
}
