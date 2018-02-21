use blockchain_hooks::{ApplicationState, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::ExploreNetworkPayload;

use hooks::State;

pub fn on_explore_network(state: ApplicationState<State>) {
    let state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");

    let payload = ExploreNetworkPayload {
        addresses: state_lock.peers.clone()
    };
    let answer = BlockchainProtocol::new()
        .set_event_code(EventCodes::ExploreNetwork)
        .set_payload(payload)
        .build();

    info!("Sending Debugger all peers.");
    state.udp.send_to(&answer, state.source).expect("Sending using UDP should be successful.");
}