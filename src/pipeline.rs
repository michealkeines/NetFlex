use crate::monitors::{Packet, TrafficMonitor};
use crate::parsers::ParserRegistry;
use crate::storage::StorageManager;
use std::sync::Arc;

pub struct TrafficPipeline {
    pub interface_monitor: Arc<dyn TrafficMonitor + Send + Sync>,
    pub parser_registry: Arc<ParserRegistry>,
    pub storage_manager: Arc<StorageManager>,
}

impl TrafficPipeline {
    pub async fn process_pipeline(&self) {
        let parser_registry = Arc::clone(&self.parser_registry);

        let ethernet_handle = tokio::spawn({
            let interface_monitor = Arc::clone(&self.interface_monitor);
            let storage_manager = Arc::clone(&self.storage_manager);

            async move {
                loop {
                    let packet: Packet = interface_monitor.capture_traffic().await;
                    let parser = parser_registry.get_parser(&packet.0);
                    let parsed_data = parser.parse(&packet);
                    storage_manager.store_data(parsed_data);

                }
            }
        });

        let _ = tokio::join!(ethernet_handle);
    }
}
