use payload::{Parser, Payload, PayloadBuilder};

/// Model for the event `RegisterAck`
///
/// ```
/// //  00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // |                                                                                               |
/// // // Blocks                                                                                       |
/// // |                                                                                               |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct GetBlocksAck {
    /// blocks of all blocks
    pub blocks: Vec<String>,
}

impl Payload for GetBlocksAck {
    fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Self {
        if !bytes.is_empty() {
            let content = Parser::string_overflow(&bytes[0..]);
            let blocks = String::from(Parser::u8_to_string(&content));
            let mut result: Vec<String> = Vec::new();

            for peer in blocks.split(", ").collect::<Vec<&str>>() {
                result.push(String::from(peer));
            }

            Self {
                blocks: result
            }
        } else {
            Self::new()
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        PayloadBuilder::new()
            .add_string_overflow(self.blocks.join(", "))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use payload::Parser;

    #[test]
    fn test_building_and_parsing() {
        let blocks = vec![String::from("172.0.0.1"), String::from("172.0.0.2")];

        let block_ack = GetBlocksAck {
            blocks: blocks.clone()
        };

        let block_ack = block_ack.to_bytes();
        let complete = Parser::parse_payload(&block_ack);
        let parsed = GetBlocksAck::parse(complete);

        assert_eq!(blocks, parsed.blocks);
    }
}