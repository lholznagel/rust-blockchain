use payload::{parser, Payload, PayloadBuilder};

/// Model for the event `RegisterAck`
///
/// ```
/// //  00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // | Address                                                                                       |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Punsh {
    /// Addresses of all peers
    pub address: String,
}

impl Punsh {
    /// Sets the peer to register at
    pub fn set_peer(mut self, peer: String) -> Self {
        self.address = peer;
        self
    }
}

impl Payload for Punsh {
    fn new() -> Self {
        Self { address: String::from("") }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Self {
        if !bytes.is_empty() {
            Self {
                address: String::from(parser::u8_to_string(&bytes[0]))
            }
        } else {
            Self::new()
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        PayloadBuilder::new()
            .add_string(self.address)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use payload::parser;

    #[test]
    fn test_building_and_parsing() {
        let address = String::from("172.0.0.1");

        let hole_puncher_ack = Punsh {
            address: address.clone()
        };

        let hole_puncher_ack = hole_puncher_ack.to_bytes();
        let complete = parser::parse_payload(&hole_puncher_ack);
        let parsed = Punsh::parse(complete);

        assert_eq!(address, parsed.address);
    }
}