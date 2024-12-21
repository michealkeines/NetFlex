use crate::extractor::ProtocolInfo;
use std::str;

// Define a trait for protocol parsers, ensuring it's Sync and Send
pub trait ProtocolParser: Sync + Send {
    fn validate(&self, data: &[u8]) -> bool;  // Validation method
    fn parse(&self, data: &[u8]) -> Option<ProtocolInfo>;
}

// Updated MQTT parser
pub struct MqttParser;

impl MqttParser {
    fn validate_mqtt(data: &[u8]) -> bool {
        if data.len() < 2 {
            return false; // MQTT packets require at least 2 bytes (Control Packet Type + Remaining Length)
        }

        let control_packet_type = data[0] >> 4; // Extract the high nibble (Control Packet Type)
        let flags = data[0] & 0x0F; // Extract the low nibble (Flags)
        let valid_packet_types = [1, 3, 5]; // CONNECT, PUBLISH, SUBSCRIBE

        // Check if Control Packet Type is valid and flags are as expected
        if !valid_packet_types.contains(&control_packet_type) || flags != 0 {
            return false;
        }

        // Decode the Remaining Length (variable-length encoding)
        let mut remaining_length = 0;
        let mut multiplier = 1;
        let mut index = 1;

        while index < data.len() && multiplier <= 128 * 128 * 128 {
            let byte = data[index];
            remaining_length += (byte & 0x7F) as usize * multiplier;
            multiplier *= 128;

            if byte & 0x80 == 0 {
                break; // End of Remaining Length field
            }
            index += 1;
        }

        // Ensure Remaining Length matches available data
        data.len() >= index + remaining_length + 1
    }

    fn parse_mqtt(data: &[u8]) -> Option<ProtocolInfo> {
        if !Self::validate_mqtt(data) {
            return None;
        }

        let control_packet_type = data[0] >> 4; // Control Packet Type
        let packet_type_str = match control_packet_type {
            1 => "CONNECT",
            3 => "PUBLISH",
            5 => "SUBSCRIBE",
            _ => "UNKNOWN",
        };

        Some(ProtocolInfo::new(
            "MQTT",
            &format!("Control Packet Type: {}", packet_type_str),
        ))
    }
}

impl ProtocolParser for MqttParser {
    fn validate(&self, data: &[u8]) -> bool {
        MqttParser::validate_mqtt(data)
    }

    fn parse(&self, data: &[u8]) -> Option<ProtocolInfo> {
        MqttParser::parse_mqtt(data)
    }
}

pub struct CoapParser;

impl CoapParser {
    fn validate_coap(data: &[u8]) -> bool {
        // CoAP packets must be at least 4 bytes long
        if data.len() < 4 {
            return false;
        }

        // Extract CoAP header fields
        let version = (data[0] & 0xC0) >> 6; // Version (bits 6-7)
        let msg_type = (data[0] & 0x30) >> 4; // Type (bits 4-5)
        let token_length = (data[0] & 0x0F) as usize; // Token Length (bits 0-3)
        let code = data[1]; // Code (1 byte)
        let message_id = u16::from_be_bytes([data[2], data[3]]); // Message ID (2 bytes)

        // Check CoAP version (must be 1)
        if version != 1 {
            return false;
        }

        // Validate message type (CON, NON, ACK, RST)
        let valid_types = [0, 1, 2, 3]; // CON, NON, ACK, RST
        if !valid_types.contains(&msg_type) {
            return false;
        }

        // Validate token length does not exceed remaining data size
        if token_length > data.len() - 4 {
            return false;
        }

        // Validate code field (class and detail)
        let class = (code & 0xE0) >> 5; // Extract class (bits 5-7)
        let detail = code & 0x1F; // Extract detail (bits 0-4)

        // Allowable classes: 0 (Empty), 2 (Success), 4 (Client Error), 5 (Server Error)
        let valid_classes = [0, 2, 4, 5];
        if !valid_classes.contains(&class) {
            return false;
        }

        // Message ID should typically be non-zero
        if message_id == 0 {
            return false;
        }

        true
    }

    fn parse_coap(data: &[u8]) -> Option<ProtocolInfo> {
        if !Self::validate_coap(data) {
            return None;
        }

        // Extract CoAP header fields
        let msg_type = (data[0] & 0x30) >> 4; // Type (bits 4-5)
        let token_length = (data[0] & 0x0F) as usize; // Token Length (bits 0-3)
        let code = data[1]; // Code (1 byte)
        let message_id = u16::from_be_bytes([data[2], data[3]]); // Message ID (2 bytes)

        // Interpret message type
        let msg_type_str = match msg_type {
            0 => "CON",
            1 => "NON",
            2 => "ACK",
            3 => "RST",
            _ => "UNKNOWN",
        };

        // Interpret code
        let class = (code & 0xE0) >> 5; // Class
        let detail = code & 0x1F; // Detail
        let code_str = format!("{}.{}", class, detail);

        Some(ProtocolInfo::new(
            "CoAP",
            &format!(
                "Type: {}, Code: {}, Message ID: {}, Token Length: {}",
                msg_type_str, code_str, message_id, token_length
            ),
        ))
    }
}

impl ProtocolParser for CoapParser {
    fn validate(&self, data: &[u8]) -> bool {
        CoapParser::validate_coap(data)
    }

    fn parse(&self, data: &[u8]) -> Option<ProtocolInfo> {
        CoapParser::parse_coap(data)
    }
}

// Define HTTP parser
pub struct HttpParser;

impl HttpParser {
    // HTTP validation: Check if the first part of the request line is a valid HTTP method
    fn validate_http(data: &[u8]) -> bool {
        //println!("data: {:?}", data);
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
