use carina_hooks::ApplicationState;
use carina_protocol::Protocol;
use carina_protocol::payload::peers::GetPeersAck;

use hooks::State;

pub fn get_peers_ack(state: ApplicationState<State>) {
    let message = Protocol::<GetPeersAck>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");
    info!("Syncing peers.");

    {
        let mut state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");

        for new_peer in message.payload.peers {
            if !new_peer.is_empty() && !state_lock.peers.contains_key(&new_peer) {
                state_lock.peers.insert(new_peer, 0);
            }
        }
    }
}