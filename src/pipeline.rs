use crate::monitors::{Packet, TrafficMonitor};
use crate::parsers::{ParserRegistry, ProtocolParser, ParsedData};
use crate::storage::StorageManager;
use std::sync::Arc;

pub struct TrafficPipeline {
    pub ethernet_monitor: Arc<dyn TrafficMonitor + Send + Sync>,
    pub wifi_monitor: Arc<dyn TrafficMonitor + Send + Sync>,
    pub ble_monitor: Arc<dyn TrafficMonitor + Send + Sync>,
    pub parser_registry: Arc<ParserRegistry>,
    pub storage_manager: Arc<StorageManager>,
}

impl TrafficPipeline {
    pub async fn process_pipeline(&self) {
        let parser_registry = Arc::clone(&self.parser_registry);

        // let ethernet_handle = tokio::spawn({
        //     let ethernet_monitor = Arc::clone(&self.ethernet_monitor);
        //     let storage_manager = Arc::clone(&self.storage_manager);
        //     let channel_manager = Arc::clone(&self.channel_manager);

        //     async move {
        //         loop {
        //             let packet = ethernet_monitor.capture_traffic().await;
        //             let parser = parser_registry.get_parser(&packet.0);
        //             let parsed_data = parser.parse(&packet);
        //             storage_manager.store_data(parsed_data).await;
        //             channel_manager.send_packet(packet);
        //         }
        //     }
        // });

        let wifi_handle = tokio::spawn({
            let wifi_monitor = Arc::clone(&self.wifi_monitor);
            let storage_manager = Arc::clone(&self.storage_manager);

            async move {
                loop {
                    let packet = wifi_monitor.capture_traffic().await;
                    let parser = parser_registry.get_parser(&packet.0);
                    let parsed_data = parser.parse(&packet);
                    storage_manager.store_data(parsed_data);
                }
            }
        });

        // let ble_handle = tokio::spawn({
        //     let ble_monitor = Arc::clone(&self.ble_monitor);
        //     let storage_manager = Arc::clone(&self.storage_manager);
        //     let channel_manager = Arc::clone(&self.channel_manager);

        //     async move {
        //         loop {
        //             let packet = ble_monitor.capture_traffic().await;
        //             let parser = parser_registry.get_parser(&packet.0);
        //             let parsed_data = parser.parse(&packet).await;
        //             storage_manager.store_data(parsed_data).await;
        //             channel_manager.send_packet(packet);
        //         }
        //     }
        // });

        let _ = tokio::join!(wifi_handle);
    }
}
