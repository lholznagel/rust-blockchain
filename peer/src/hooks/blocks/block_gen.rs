use carina_hooks::{as_number, MessageState, HookCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::blocks::{BlockGen, BlockFound, HashVal};

use hooks::State;

use crypto::digest::Digest;
use crypto::sha3::Sha3;

pub fn block_gen(state: MessageState<State>) {
    let mut nacl = {
        let state_lock = state.state.lock().expect("Locking the mutex should be successful.");
        state_lock.nacl.clone()
    };
    let peers = {
        let state_lock = state.state.lock().expect("Locking the mutex should be successful.");
        state_lock.peers.clone()
    };
    let message = Protocol::<BlockGen>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");

    {
        let mut state_lock = state.state.lock().expect("Locking the mutex should be successful.");

        if state_lock.is_calculating {
            return;
        } else {
            state_lock.is_calculating = true;
        }
    }

    let hash;
    let mut nonce = 0;

    info!("Starting generating a new block.");
    loop {
        let mut generated_block = String::from("");
        generated_block.push_str(&message.payload.content);
        generated_block.push_str(&message.payload.index.to_string());
        generated_block.push_str(&message.payload.timestamp.to_string());
        generated_block.push_str(&message.payload.prev);
        generated_block.push_str(&nonce.to_string());

        let mut hasher = Sha3::sha3_256();
        hasher.input_str(generated_block.as_str());
        let hex = hasher.result_str();

        if message.payload.sign_key == &hex[..message.payload.sign_key.len()] {
            hash = hex.clone();
            break;
        } else {
            nonce += 1;
        }
    }

    {
        let mut state_lock = state.state.lock().expect("Locking the mutex should be successful.");
        state_lock.is_calculating = false;
        state_lock.current_block = BlockFound {
            content: message.payload.content.clone(),
            timestamp: message.payload.timestamp.clone(),
            index: message.payload.index.clone(),
            prev: message.payload.prev.clone(),
            nonce: nonce.clone(),
            hash: hash.clone()
        };
    }

    info!("[BLOCK GEN] Found hash! {:?}", hash);
    let payload = HashVal {
        content: message.payload.content,
        timestamp: message.payload.timestamp,
        index: message.payload.index,
        prev: message.payload.prev,
        nonce: nonce
    };

    debug!("[BLOCK GEN] Sending hash to other peers for validation.");
    for (peer, (public_key, _, _)) in peers {
        let message = Protocol::<HashVal>::new()
            .set_event_code(as_number(HookCodes::HashVal))
            .set_payload(payload.clone())
            .build(&mut nacl, &public_key);

        state.udp.send_to(message.as_slice(), peer)
            .expect("Sending using UDP should be successful.");
    }
}