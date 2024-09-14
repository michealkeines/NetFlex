use crate::monitors::TrafficMonitor;
use crate::packet::ClonablePacket as Packet;
use crate::parsers::ParserRegistry;
use crate::storage::StorageManager;
use crate::extractor::{InformationExtractor, PacketInfo}; // Assuming the InformationExtractor is defined in this module.
use std::sync::Arc;
use dashmap::DashMap;

pub struct TrafficPipeline {
    pub interface_monitor: Arc<dyn TrafficMonitor + Send + Sync>,
    pub parser_registry: Arc<ParserRegistry>,
    pub storage_manager: Arc<StorageManager>,
    pub info_extractor: Arc<InformationExtractor>,
}

impl TrafficPipeline {
    pub async fn process_pipeline(&self) {
        let parser_registry = Arc::clone(&self.parser_registry);
        let info_extractor = Arc::clone(&self.info_extractor);  // Cloning the InformationExtractor reference

        let ethernet_handle = tokio::spawn({
            let interface_monitor = Arc::clone(&self.interface_monitor);
            let storage_manager = Arc::clone(&self.storage_manager);

            async move {
                loop {
                    let packet: Packet = interface_monitor.capture_traffic().await;
                    println!("data: {packet:?}");

                    // Use the parser registry to parse the packet
                    let parser = parser_registry.get_parser(&packet.raw);
                    let parsed_data = parser.parse(&packet);

                    // Extract information from the packet
                    info_extractor.extract_and_store(&packet);

                    // Store the parsed data in the storage manager
                   //storage_manager.store_data(parsed_data);
                }
            }
        });

        let _ = tokio::join!(ethernet_handle);
    }
}

