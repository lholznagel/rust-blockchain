use blockchain_hooks::{ApplicationState, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::{SyncBlocksAck, SyncBlocksReq};

use hooks::State;

use std::path::Path;

pub fn on_sync_blocks_ack(state: ApplicationState<State>) {
    let message = BlockchainProtocol::<SyncBlocksAck>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");

    for block in message.payload.blocks {
        if !Path::new(&format!("./block_data/{}", block)).exists() {
            let payload = SyncBlocksReq {
                block
            };
            let message = BlockchainProtocol::new()
                .set_event_code(EventCodes::SyncBlocksReq)
                .set_payload(payload)
                .build();

            state.udp.send_to(message.as_slice(), state.source.clone())
                .expect("Sending using UDP should be successful.");
        }
    }
}