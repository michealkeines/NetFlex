mod monitors;
mod pipeline;
mod config;  // Import the config module
mod packet;
mod extractor;

mod probe;

use std::sync::Arc;
use extractor::InformationExtractor;
use tokio::task::JoinSet;
use pipeline::TrafficPipeline;
use monitors::InterfaceMonitor;
use config::{load_config, Config};  // Import load_config function and Config struct

#[tokio::main]
async fn main() {
    // Load config from a custom file
    let config: Config = load_config("custom_config.json").await;

    let info_extractor = Arc::new(InformationExtractor::new());

    // Optional: Access future settings (log level, etc.)
    if let Some(settings) = &config.settings {
        if let Some(log_level) = &settings.log_level {
            println!("Log level set to: {}", log_level);
            // Set up logging if needed
        }
        if let Some(storage_path) = &settings.storage_path {
            println!("Storage path set to: {}", storage_path);
            // Use the storage path if needed
        }
    }

    // Create a JoinSet to manage tasks
    let mut join_set = JoinSet::new();

    // Start pipelines for each interface in parallel
    for interface in config.network.interfaces {
        let interface_monitor = Arc::new(InterfaceMonitor { device_name: interface.clone() });
        let pipeline = TrafficPipeline {
            interface_monitor,
            info_extractor: Arc::clone(&info_extractor),
        };

        // Spawn each pipeline and add to JoinSet
        join_set.spawn(async move {
            pipeline.process_pipeline().await;
        });
    }

    // Process tasks as they complete
    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(_) => println!("Pipeline task completed successfully."),
            Err(e) => eprintln!("Pipeline task failed: {:?}", e),
        }
    }
}
