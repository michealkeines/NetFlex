mod monitors;
mod parsers;
mod pipeline;
mod storage;

mod protocols;

use std::sync::Arc;
use pipeline::TrafficPipeline;
use monitors::{EthernetMonitor, WiFiMonitor, BLEMonitor};
use parsers::ParserRegistry;
use storage::StorageManager;

#[tokio::main]
async fn main() {
    let ethernet_monitor = Arc::new(EthernetMonitor { device_name: "eth0".to_string() });
    let wifi_monitor = Arc::new(WiFiMonitor { device_name: "en0".to_string() });
    let ble_monitor = Arc::new(BLEMonitor { device_name: "ble0".to_string() });

    let parser_registry = Arc::new(ParserRegistry::new());

    let storage_manager = Arc::new(StorageManager);

    let pipeline = TrafficPipeline {
        ethernet_monitor,
        wifi_monitor,
        ble_monitor,
        parser_registry,
        storage_manager,
    };

    pipeline.process_pipeline().await;
}
