use std::sync::Arc;
use dashmap::DashSet;
use tokio::time::{timeout, Duration};

use crate::extractor::PacketInfo;

use tokio::net::UdpSocket;
use std::net::SocketAddr;
use coap_lite::{Packet, CoapRequest};

use reqwest::Client; // For HTTP requests
use rumqttc::{MqttOptions, AsyncClient, QoS}; // For MQTT

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValidResponse {
    pub protocol: String, // e.g., "HTTP", "MQTT", "COAP"
    pub version: Option<String>, // Version of the protocol, if applicable
    pub response_metadata: Option<String>, // Metadata extracted from the response, e.g., server info, headers
    pub packet_info: PacketInfo, // The original packet information
}

pub struct ProtocolProber {
    db: Arc<DashSet<PacketInfo>>, // Shared DB with unique PacketInfo
    pub valid_responses: Arc<DashSet<ValidResponse>>, // Store the valid responses here
}

impl ProtocolProber {
    pub fn new(db: Arc<DashSet<PacketInfo>>) -> Self {
        Self {
            db,
            valid_responses: Arc::new(DashSet::new()),
        }
    }

    // The main function that iterates over PacketInfo and probes different protocols
    pub async fn active_probe(&self) {
        let tasks: Vec<_> = self.db.iter().map(|packet| {
            let probe_http = self.probe_http(packet.clone());
            let probe_coap = self.probe_coap(packet.clone());
            let probe_mqtt = self.probe_mqtt(packet.clone());

            async move {
                tokio::join!(probe_http, probe_coap, probe_mqtt);
            }
        }).collect();

        // Run all probes concurrently
        for task in tasks {
            task.await; // Await each task to ensure they complete
        }
    }

    // Example HTTP probe function (using reqwest)
    async fn probe_http(&self, packet: PacketInfo) -> Option<ValidResponse> {
        if let Some(ip) = &packet.dst_ip {
            if let Some(port) = packet.dst_port {
                let client = Client::new();
                let url = format!("http://{}:{}/", ip, port);
                
                // Timeout and handle request asynchronously
                if let Ok(response) = timeout(Duration::from_secs(2), client.get(&url).send()).await {
                    if let Ok(response) = response {
                            return Some(ValidResponse {
                                protocol: "HTTP".to_string(),
                                version: Some(format!("{:?}", response.version())),
                                response_metadata: Some(format!("{:?}", response.headers())),
                                packet_info: packet.clone(),
                            });
                    }
                }
            }
        }
        None
    }

    async fn probe_coap(&self, packet: PacketInfo) -> Option<ValidResponse> {
        // Check if the destination IP and port are available in the packet information
        if let Some(ip) = &packet.dst_ip {
            if let Some(port) = packet.dst_port {
                // Prepare the CoAP request packet
                let mut coap_request: CoapRequest<SocketAddr> = CoapRequest::new();
                coap_request.set_path("/.well-known/core"); // Standard CoAP discovery path
    
                // Attempt to convert the CoAP request into a byte array
                let payload: Vec<u8> = match coap_request.message.to_bytes() {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        println!("Error converting CoAP request to bytes: {:?}", e);
                        return None;
                    }
                };
    
                // Bind to a local UDP socket
                let socket: UdpSocket = match UdpSocket::bind("0.0.0.0:0").await {
                    Ok(s) => s,
                    Err(e) => {
                        println!("Error binding UDP socket: {:?}", e);
                        return None;
                    }
                };
    
                // Attempt to send the CoAP request to the target IP and port
                if let Err(e) = socket.send_to(&payload, (ip.as_str(), port)).await {
                    println!("Error sending CoAP request: {:?}", e);
                    return None;
                }
    
                // Buffer to store the incoming response
                let mut buf: [u8; 512] = [0; 512];
    
                // Set a timeout for receiving the response
                match timeout(Duration::from_secs(2), socket.recv_from(&mut buf)).await {
                    // If the timeout succeeds and a packet is received
                    Ok(Ok((len, _src))) => {
                        // Attempt to parse the received CoAP response packet
                        match Packet::from_bytes(&buf[..len]) {
                            Ok(response_packet) => {
                                // Construct a valid response with protocol details
                                return Some(ValidResponse {
                                    protocol: "COAP".to_string(),
                                    version: Some("1.0".to_string()), // Assuming CoAP version 1.0
                                    response_metadata: Some(format!("{:?}", response_packet)),
                                    packet_info: packet.clone(),
                                });
                            }
                            Err(e) => {
                                println!("Error parsing CoAP response packet: {:?}", e);
                            }
                        }
                    }
                    // Timeout case
                    Ok(Err(e)) => {
                        println!("Error receiving CoAP response: {:?}", e);
                    }
                    // Timeout exceeded
                    Err(_) => {
                        println!("Timed out waiting for CoAP response");
                    }
                }
            }
        }
        // Return None if any error occurs or packet data is insufficient
        None
    }
    
    // Example MQTT probe function (using rumqttc)
    async fn probe_mqtt(&self, packet: PacketInfo) -> Option<ValidResponse> {
        if let Some(ip) = &packet.dst_ip {
            if let Some(port) = packet.dst_port {
                let mut mqttoptions = MqttOptions::new("test-client", ip.as_str(), port);
                mqttoptions.set_keep_alive(Duration::from_secs(5));
                
                let (client,eventloop) = AsyncClient::new(mqttoptions, 10);
                let response = timeout(Duration::from_secs(2), client.subscribe("test", QoS::AtMostOnce)).await;
                
                if response.is_ok() {
                    return Some(ValidResponse {
                        protocol: "MQTT".to_string(),
                        version: Some("3.1.1".to_string()), // MQTT 3.1.1 or 5.0
                        response_metadata: Some("Subscription successful".to_string()),
                        packet_info: packet.clone(),
                    });
                }
            }
        }
        None
    }
}
