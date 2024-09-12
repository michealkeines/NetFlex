use pcap::Capture;
use async_trait::async_trait;
use tokio::task;

#[derive(Debug, Clone)]
pub struct Packet(pub Vec<u8>);

#[async_trait]
pub trait TrafficMonitor {
    async fn capture_traffic(&self) -> Packet;
}

pub struct EthernetMonitor {
    pub device_name: String,
}

#[async_trait]
impl TrafficMonitor for EthernetMonitor {
    async fn capture_traffic(&self) -> Packet {
        task::spawn_blocking({
            let device_name = self.device_name.clone();
            move || {
                let mut cap = Capture::from_device(&device_name[..]).unwrap()
                    .immediate_mode(true)
                    .open().unwrap();
                let packet = cap.next_packet().unwrap();
                Packet(packet.data.to_vec())
            }
        }).await.unwrap()
    }
}

pub struct WiFiMonitor {
    pub device_name: String,
}

#[async_trait]
impl TrafficMonitor for WiFiMonitor {
    async fn capture_traffic(&self) -> Packet {
        task::spawn_blocking({
            let device_name = self.device_name.clone();
            move || {
                let mut cap = Capture::from_device(&device_name[..]).unwrap()
                    .immediate_mode(true)
                    .open().unwrap();
                let packet = cap.next_packet().unwrap();
                Packet(packet.data.to_vec())
            }
        }).await.unwrap()
    }
}

pub struct BLEMonitor {
    pub device_name: String,
}

#[async_trait]
impl TrafficMonitor for BLEMonitor {
    async fn capture_traffic(&self) -> Packet {
        task::spawn_blocking({
            let device_name = self.device_name.clone();
            move || {
                let mut cap = Capture::from_device(&device_name[..]).unwrap()
                    .immediate_mode(true)
                    .open().unwrap();
                let packet = cap.next_packet().unwrap();
                Packet(packet.data.to_vec())
            }
        }).await.unwrap()
    }
}
