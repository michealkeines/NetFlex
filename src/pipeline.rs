use crate::monitors::TrafficMonitor;
use crate::packet::ClonablePacket as Packet;
use crate::probe::ProtocolProber;
use crate::extractor::{InformationExtractor, PacketInfo}; // Assuming the InformationExtractor is defined in this module.
use std::sync::Arc;

pub struct TrafficPipeline {
    pub interface_monitor: Arc<dyn TrafficMonitor + Send + Sync>,
    pub info_extractor: Arc<InformationExtractor>,
}

impl TrafficPipeline {
    pub async fn process_pipeline(&self) {
        let info_extractor = Arc::clone(&self.info_extractor);  // Cloning the InformationExtractor reference

        let ethernet_handle = tokio::spawn({
            let interface_monitor = Arc::clone(&self.interface_monitor);

            async move {
                loop {
                    let packet: Packet = interface_monitor.capture_traffic().await;

                    // Extract information from the packet
                    info_extractor.extract_and_store(&packet);
                    let prober = ProtocolProber::new(info_extractor.db.clone());
    
                    // Run active probing
                    prober.active_probe().await;
                    println!("proobing over");
                    // Access valid responses
                    for response in prober.valid_responses.iter() {
                        println!("Valid protocol: {:?}", response.response_metadata);
                    }
                }
            }
        });

        let _ = tokio::join!(ethernet_handle);
    }
}

