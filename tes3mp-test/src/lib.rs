use crate::plugin::{Events, LOG_INFO};

mod plugin;

struct Server;

impl Events for Server {
    fn new() -> Self {
        Server
    }

    fn on_any(&self, event: &str) {
        plugin::log_message(LOG_INFO, format!("Event triggered: {}", event).as_str());
    }

    fn on_server_init(&self) {
        plugin::log_message(plugin::LOG_WARN, "Hello from Rust :3");
    }

    fn on_server_post_init(&self) {
        plugin::log_message(plugin::LOG_FATAL, "Hi!?");
    }
}

use_events!(Server);
