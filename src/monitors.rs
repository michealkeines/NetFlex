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