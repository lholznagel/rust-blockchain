use carina_core::Config;
use carina_core::Event;
use commands::console::InternalState;
use protocol_builder_parser::Parser;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

pub struct NewBlockContent {
    internal_state: Arc<Mutex<InternalState>>
}

impl NewBlockContent {
    pub fn new(internal_state: Arc<Mutex<InternalState>>) -> Self {
        Self {
            internal_state
        }
    }
}

impl Event for NewBlockContent {
    fn execute(&mut self, _: UdpSocket, _: String, _: &mut Config, buffer: &[u8]) {
        let parsed = Parser::parse_payload(&buffer);
        let code = match Parser::to_string(&parsed[0].clone()) {
            Ok(val) => val,
            Err(e)  => {
                error!("[CONSOLE_NEW_BLOCK_CONTENT] Error getting code {}", e);
                String::new()
            }
        };
        let content = match Parser::to_string(&parsed[1].clone()) {
            Ok(val) => val,
            Err(e)  => {
                error!("[CONSOLE_NEW_BLOCK_CONTENT] Error getting content {}", e);
                String::new()
            }
        };

        if !code.is_empty() || !content.is_empty() {
            match self.internal_state.lock() {
                Ok(mut state) => {
                    state.content.insert(code, content);
                    debug!("[CONSOLE_NEW_BLOCK_CONTENT] Added new content");
                },
                Err(e)        => error!("[CONSOLE_NEW_BLOCK_CONTENT] Error locking state. {}", e)
            };
        }
    }
}