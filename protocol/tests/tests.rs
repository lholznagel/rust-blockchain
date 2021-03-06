#[macro_use]
extern crate quickcheck;
extern crate carina_protocol;

use carina_protocol::Protocol;
use carina_protocol::nacl::Nacl;
use carina_protocol::payload::*;
use carina_protocol::payload::blocks::*;

quickcheck! {
    fn test_punsh(address: String) -> bool {
        let mut our_nacl = Nacl::new();
        let their_nacl = Nacl::new();
        let address = address;

        let payload = Punsh {
            address: address.clone()
        };

        let blockchain_protocol = Protocol::<Punsh>::new()
            .set_event_code(2)
            .set_payload(payload)
            .build(&mut our_nacl, &their_nacl.get_public_key());

        let blockchain_protocol = carina_protocol::parse_encrypted(&blockchain_protocol, &their_nacl, &our_nacl.get_public_key());
        let blockchain_parsed = Protocol::<Punsh>::from_bytes(&blockchain_protocol).unwrap();
        assert_eq!(address, blockchain_parsed.payload.address);
        true
    }
}

quickcheck! {
    fn test_get_block(block: String) -> bool {
        let mut our_nacl = Nacl::new();
        let their_nacl = Nacl::new();
        let block = block;

        let payload = GetBlock {
            block: block.clone()
        };

        let blockchain_protocol = Protocol::<GetBlock>::new()
            .set_event_code(130)
            .set_payload(payload)
            .build(&mut our_nacl, &their_nacl.get_public_key());

        let blockchain_protocol = carina_protocol::parse_encrypted(&blockchain_protocol, &their_nacl, &our_nacl.get_public_key());
        let blockchain_parsed = Protocol::<GetBlock>::from_bytes(&blockchain_protocol).unwrap();
        assert_eq!(block, blockchain_parsed.payload.block);
        true
    }
}

quickcheck! {
    fn test_get_block_ack(filename: String, index: u64, timestamp: i64, nonce: u64, prev: String, hash: String, content: String) -> bool {
        let mut our_nacl = Nacl::new();
        let their_nacl = Nacl::new();
        let filename = filename;
        let index = index;
        let timestamp = timestamp;
        let nonce = nonce;
        let prev = prev;
        let hash = hash;
        let content = content;

        let payload = GetBlockAck {
            filename: filename.clone(),
            index: index.clone(),
            timestamp: timestamp.clone(),
            nonce: nonce.clone(),
            prev: prev.clone(),
            hash: hash.clone(),
            content: content.clone()
        };

        let blockchain_protocol = Protocol::<GetBlockAck>::new()
            .set_event_code(131)
            .set_payload(payload)
            .build(&mut our_nacl, &their_nacl.get_public_key());

        let blockchain_protocol = carina_protocol::parse_encrypted(&blockchain_protocol, &their_nacl, &our_nacl.get_public_key());
        let blockchain_parsed = Protocol::<GetBlockAck>::from_bytes(&blockchain_protocol).unwrap();
        assert_eq!(filename, blockchain_parsed.payload.filename);
        assert_eq!(index, blockchain_parsed.payload.index);
        assert_eq!(timestamp, blockchain_parsed.payload.timestamp);
        assert_eq!(nonce, blockchain_parsed.payload.nonce);
        assert_eq!(prev, blockchain_parsed.payload.prev);
        assert_eq!(hash, blockchain_parsed.payload.hash);
        assert_eq!(content, blockchain_parsed.payload.content);
        true
    }
}

quickcheck! {
    fn test_block_data(unique_key: String, content: String) -> bool {
        let mut our_nacl = Nacl::new();
        let their_nacl = Nacl::new();
        let content = content;

        let payload = BlockData {
            unique_key: unique_key.clone(),
            content: content.clone()
        };

        let blockchain_protocol = Protocol::<BlockData>::new()
            .set_event_code(132)
            .set_payload(payload)
            .build(&mut our_nacl, &their_nacl.get_public_key());

        let blockchain_protocol = carina_protocol::parse_encrypted(&blockchain_protocol, &their_nacl, &our_nacl.get_public_key());
        let blockchain_parsed = Protocol::<BlockData>::from_bytes(&blockchain_protocol).unwrap();
        assert_eq!(unique_key, blockchain_parsed.payload.unique_key);
        assert_eq!(content, blockchain_parsed.payload.content);
        true
    }
}

quickcheck! {
    fn test_block_gen(index: u64, timestamp: i64, prev: String, sign_key: String, content: String) -> bool {
        let mut our_nacl = Nacl::new();
        let their_nacl = Nacl::new();
        let index = index;
        let timestamp = timestamp;
        let sign_key = sign_key;
        let prev = prev;
        let content = content;

        let payload = BlockGen {
            index: index.clone(),
            timestamp: timestamp.clone(),
            sign_key: sign_key.clone(),
            prev: prev.clone(),
            content: content.clone()
        };

        let blockchain_protocol = Protocol::<BlockGen>::new()
            .set_event_code(133)
            .set_payload(payload)
            .build(&mut our_nacl, &their_nacl.get_public_key());

        let blockchain_protocol = carina_protocol::parse_encrypted(&blockchain_protocol, &their_nacl, &our_nacl.get_public_key());
        let blockchain_parsed = Protocol::<BlockGen>::from_bytes(&blockchain_protocol).unwrap();
        assert_eq!(index, blockchain_parsed.payload.index);
        assert_eq!(content, blockchain_parsed.payload.content);
        assert_eq!(timestamp, blockchain_parsed.payload.timestamp);
        assert_eq!(sign_key, blockchain_parsed.payload.sign_key);
        assert_eq!(prev, blockchain_parsed.payload.prev);
        true
    }
}

quickcheck! {
    fn test_block_found(index: u64, timestamp: i64, nonce: u64, hash: String, prev: String, content: String) -> bool {
        let mut our_nacl = Nacl::new();
        let their_nacl = Nacl::new();
        let index = index;
        let timestamp = timestamp;
        let nonce = nonce;
        let prev = prev;
        let hash = hash;
        let content = content;

        let payload = BlockFound {
            index: index.clone(),
            timestamp: timestamp.clone(),
            nonce: nonce.clone(),
            prev: prev.clone(),
            hash: hash.clone(),
            content: content.clone()
        };

        let blockchain_protocol = Protocol::<BlockFound>::new()
            .set_event_code(134)
            .set_payload(payload)
            .build(&mut our_nacl, &their_nacl.get_public_key());

        let blockchain_protocol = carina_protocol::parse_encrypted(&blockchain_protocol, &their_nacl, &our_nacl.get_public_key());
        let blockchain_parsed = Protocol::<BlockFound>::from_bytes(&blockchain_protocol).unwrap();
        assert_eq!(index, blockchain_parsed.payload.index);
        assert_eq!(timestamp, blockchain_parsed.payload.timestamp);
        assert_eq!(nonce, blockchain_parsed.payload.nonce);
        assert_eq!(prev, blockchain_parsed.payload.prev);
        assert_eq!(hash, blockchain_parsed.payload.hash);
        assert_eq!(content, blockchain_parsed.payload.content);
        true
    }
}

quickcheck! {
    fn test_hash_val(index: u64, timestamp: i64, nonce: u64, prev: String, content: String) -> bool {
        let mut our_nacl = Nacl::new();
        let their_nacl = Nacl::new();
        let index = index;
        let nonce = nonce;
        let timestamp = timestamp;
        let prev = prev;
        let content = content;

        let payload = HashVal {
            index: index.clone(),
            timestamp: timestamp.clone(),
            nonce: nonce.clone(),
            prev: prev.clone(),
            content: content.clone()
        };

        let blockchain_protocol = Protocol::<HashVal>::new()
            .set_event_code(135)
            .set_payload(payload)
            .build(&mut our_nacl, &their_nacl.get_public_key());

        let blockchain_protocol = carina_protocol::parse_encrypted(&blockchain_protocol, &their_nacl, &our_nacl.get_public_key());
        let blockchain_parsed = Protocol::<HashVal>::from_bytes(&blockchain_protocol).unwrap();
        assert_eq!(index, blockchain_parsed.payload.index);
        assert_eq!(content, blockchain_parsed.payload.content);
        assert_eq!(timestamp, blockchain_parsed.payload.timestamp);
        assert_eq!(nonce, blockchain_parsed.payload.nonce);
        assert_eq!(prev, blockchain_parsed.payload.prev);
        true
    }
}

quickcheck! {
    fn test_hash_val_ack(index: u64, hash: String) -> bool {
        let mut our_nacl = Nacl::new();
        let their_nacl = Nacl::new();
        let index = index;
        let hash = hash;

        let payload = HashValAck {
            index: index.clone(),
            hash: hash.clone()
        };

        let blockchain_protocol = Protocol::<HashValAck>::new()
            .set_event_code(136)
            .set_payload(payload)
            .build(&mut our_nacl, &their_nacl.get_public_key());

        let blockchain_protocol = carina_protocol::parse_encrypted(&blockchain_protocol, &their_nacl, &our_nacl.get_public_key());
        let blockchain_parsed = Protocol::<HashValAck>::from_bytes(&blockchain_protocol).unwrap();
        assert_eq!(index, blockchain_parsed.payload.index);
        assert_eq!(hash, blockchain_parsed.payload.hash);
        true
    }
}