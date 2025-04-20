mod monitors;
mod pipeline;
mod config;
mod packet;
mod extractor;
mod protocol;
mod probe;

use std::sync::Arc;
use extractor::InformationExtractor;
use tokio::task::JoinSet;
use pipeline::TrafficPipeline;
use monitors::{FileMonitor, InterfaceMonitor};
use config::{load_config, Config};

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

    // Determine mode: live monitoring or file monitoring based on config
    let mode = config.network.mode.as_deref().unwrap_or("live");

    // Create a JoinSet to manage tasks
    let mut join_set = JoinSet::new();

    if mode == "live" {
        println!("Running in live monitoring mode. this is getting logged in plain text");
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
    } else if mode == "file" {
        println!("Running in file monitoring mode.");
        // Add a FileMonitor pipeline if a PCAP file is provided in the config
        if let Some(pcap_file) = &config.network.pcap_file {
            let file_monitor = Arc::new(FileMonitor::new(pcap_file.clone()));
            let pipeline = TrafficPipeline {
                interface_monitor: file_monitor,
                info_extractor: Arc::clone(&info_extractor),
            };

            // Spawn FileMonitor pipeline and add to JoinSet
            join_set.spawn(async move {
                pipeline.process_pipeline().await;
            });
        } else {
            eprintln!("Error: No PCAP file specified in configuration for file monitoring mode.");
            return;
        }
    } else {
        eprintln!("Error: Unknown mode '{}'. Use 'live' or 'file'.", mode);
        return;
    }

    // Process tasks as they complete
    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(_) => println!("Pipeline task completed successfully."),
            Err(e) => eprintln!("Pipeline task failed: {:?}", e),
        }
    }
}
