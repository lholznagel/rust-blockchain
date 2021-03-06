use carina_hooks::{as_number, MessageState, HookCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::blocks::{GetBlocksAck, GetBlock};

use hooks::State;

use std::path::Path;

pub fn get_blocks_ack(state: MessageState<State>) {
    let mut nacl = {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.nacl.clone()
    };
    let peers = {
        let state_lock = state.state.lock().expect("Locking the mutex should be successful.");
        state_lock.peers.clone()
    };
    let storage = {
        let state_lock = state.state.lock().expect("Locking the mutex should be successful.");
        state_lock.storage.clone()
    };

    let message = Protocol::<GetBlocksAck>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");

    let contacting_peer = peers.get(&state.source.clone()).unwrap();

    for block in message.payload.blocks {
        if !Path::new(&format!("{}/{}", storage, block)).exists() {
            let payload = GetBlock {
                block
            };
            let message = Protocol::new()
                .set_event_code(as_number(HookCodes::GetBlock))
                .set_payload(payload)
                .build(&mut nacl, &contacting_peer.0);

            state.udp.send_to(message.as_slice(), state.source.clone())
                .expect("Sending using UDP should be successful.");
        }
    }
}