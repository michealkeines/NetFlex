use crate::extractor::ProtocolInfo;
use std::str;

// Define a trait for protocol parsers, ensuring it's Sync and Send
pub trait ProtocolParser: Sync + Send {
    fn validate(&self, data: &[u8]) -> bool;  // Validation method
    fn parse(&self, data: &[u8]) -> Option<ProtocolInfo>;
}

// Define MQTT parser
pub struct MqttParser;

impl MqttParser {
    // MQTT validation: check if the first byte is a valid MQTT packet type
    fn validate_mqtt(data: &[u8]) -> bool {
        // MQTT control packets start with a valid control packet byte
        if data.is_empty() {
            return false;
        }
        let packet_type = data[0];
        packet_type == 1 || packet_type == 3 || packet_type == 5 // Check for CONNECT, PUBLISH, SUBSCRIBE
    }
}

impl ProtocolParser for MqttParser {
    fn validate(&self, data: &[u8]) -> bool {
        MqttParser::validate_mqtt(data)
    }

    fn parse(&self, data: &[u8]) -> Option<ProtocolInfo> {
        if self.validate(data) {
            let packet_type = data[0];
            let packet_type_str = match packet_type {
                1 => "CONNECT",
                3 => "PUBLISH",
                5 => "SUBSCRIBE",
                _ => "UNKNOWN",
            };
            Some(ProtocolInfo::new(
                "MQTT",
                &format!("Packet Type: {}", packet_type_str),
            ))
        } else {
            None
        }
    }
}

// Define CoAP parser
pub struct CoapParser;

impl CoapParser {
    // CoAP validation: check if the first byte is a valid CoAP type
    fn validate_coap(data: &[u8]) -> bool {
        if data.is_empty() {
            return false;
        }
        let coap_type = data[0];
        // Valid CoAP types
        coap_type == 0 || coap_type == 1 || coap_type == 2 || coap_type == 3
    }
}

impl ProtocolParser for CoapParser {
    fn validate(&self, data: &[u8]) -> bool {
        CoapParser::validate_coap(data)
    }

    fn parse(&self, data: &[u8]) -> Option<ProtocolInfo> {
        if self.validate(data) {
            let coap_type = data[0];
            let coap_type_str = match coap_type {
                0 => "CON",
                1 => "NON",
                2 => "ACK",
                3 => "RST",
                _ => "UNKNOWN",
            };
            Some(ProtocolInfo::new("CoAP", &format!("CoAP Type: {}", coap_type_str)))
        } else {
            None
        }
    }
}

// Define HTTP parser
pub struct HttpParser;

impl HttpParser {
    // HTTP validation: Check if the first part of the request line is a valid HTTP method
    fn validate_http(data: &[u8]) -> bool {
        if let Ok(http_str) = str::from_utf8(data) {
            let first_line = http_str.lines().next().unwrap_or("");
            // Check if the first word is a valid HTTP method
            return first_line.starts_with("GET") || first_line.starts_with("POST") ||
                   first_line.starts_with("PUT") || first_line.starts_with("DELETE");
        }
        false
    }
}

impl ProtocolParser for HttpParser {
    fn validate(&self, data: &[u8]) -> bool {
        HttpParser::validate_http(data)
    }

    fn parse(&self, data: &[u8]) -> Option<ProtocolInfo> {
        if self.validate(data) {
            if let Ok(http_str) = str::from_utf8(data) {
                let first_line = http_str.lines().next().unwrap_or("");
                Some(ProtocolInfo::new("HTTP", first_line))
            } else {
                None
            }
        } else {
            None
        }
    }
}
