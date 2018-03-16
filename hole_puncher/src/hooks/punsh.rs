use blockchain_hooks::{as_number, ApplicationState, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::Punsh;

use hooks::State;

pub fn punsh(state: ApplicationState<State>) {
    let message = BlockchainProtocol::<Punsh>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");

    let payload = Punsh {
        address: state.source
    };

    let result = BlockchainProtocol::<Punsh>::new()
        .set_payload(payload)
        .set_event_code(as_number(EventCodes::Punsh))
        .build();

    state.udp.send_to(&result, message.payload.address)
        .expect("Sending using UDP should be successful.");
}