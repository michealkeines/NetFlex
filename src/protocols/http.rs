use crate::{monitors::Packet, parsers::{ParsedData, Protocol, ProtocolParser}};
use httparse::{Request, Status};

#[derive(Clone, Copy)]
pub struct HttpParser;

impl ProtocolParser for HttpParser {
    fn parse(&self, packet: &Packet) -> ParsedData {
        // Convert packet bytes to a vector of u8
        let packet_bytes = &packet.0;

        // Create a request object
        let mut headers = [httparse::EMPTY_HEADER; 16]; // Limit of 16 headers
        let mut req = Request::new(&mut headers);

        // Parse the HTTP request from the packet data
        match req.parse(packet_bytes) {
            Ok(Status::Complete(_)) => {
                // Successfully parsed HTTP request
                let method = req.method.unwrap_or("UNKNOWN");
                let path = req.path.unwrap_or("UNKNOWN");
                let version = req.version.unwrap_or(0);

                let protocol_data = format!("HTTP/{} {} {}", version, method, path);
                
                ParsedData {
                    protocol: Protocol::Http(protocol_data),
                }
            }
            Ok(Status::Partial) => {
                // Packet data is incomplete for parsing
                ParsedData {
                    protocol: Protocol::Http("Incomplete HTTP packet".to_string()),
                }
            }
            Err(_) => {
                // Failed to parse HTTP
                ParsedData {
                    protocol: Protocol::Http("Invalid HTTP packet".to_string()),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monitors::Packet;
    use crate::parsers::{ProtocolParser, Protocol};

    #[test]
    fn test_valid_http_request() {
        let http_data = b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let packet = Packet(http_data.to_vec());
        let parser = HttpParser;
        let result = parser.parse(&packet);

        if let Protocol::Http(parsed) = result.protocol {
            assert_eq!(parsed, "HTTP/1 GET /");
        } else {
            panic!("Expected HTTP protocol");
        }
    }

    #[test]
    fn test_invalid_http_request() {
        let invalid_data = b"00000";
        let packet = Packet(invalid_data.to_vec());
        let parser = HttpParser;
        let result = parser.parse(&packet);

        if let Protocol::Http(parsed) = result.protocol {
            assert_eq!(parsed, "Incomplete HTTP packet");
        } else {
            panic!("Expected HTTP protocol");
        }
    }

    #[test]
    fn test_partial_http_request() {
        let partial_data = b"GET / HTTP/1.1\r\nHost: example.com";
        let packet = Packet(partial_data.to_vec());
        let parser = HttpParser;
        let result = parser.parse(&packet);

        if let Protocol::Http(parsed) = result.protocol {
            assert_eq!(parsed, "Incomplete HTTP packet");
        } else {
            panic!("Expected HTTP protocol");
        }
    }
}

