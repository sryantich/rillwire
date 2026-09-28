//! Transport-independent, bounded state. No network, credentials, or UI dependencies.
pub mod cache;
pub mod heartbeat;
pub mod rate_limit;

pub use cache::{CacheError, Message, MessageCache, MessageKey};

/// Commands express intent. A future authenticated adapter must acknowledge delivery.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    SendMessage { channel_id: u64, content: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    MessageCreated(Message),
    MessageDeleted(MessageKey),
    Disconnected,
}

/// Synchronous boundary for the offline demo; async adapters will use bounded channels.
pub trait Backend {
    fn execute(&mut self, command: Command) -> Result<Event, String>;
}

#[derive(Default)]
pub struct DemoBackend {
    next_id: u64,
}

impl Backend for DemoBackend {
    fn execute(&mut self, command: Command) -> Result<Event, String> {
        let Command::SendMessage {
            channel_id,
            content,
        } = command;
        let content = content.trim();
        if content.is_empty() || content.chars().count() > 2000 {
            return Err("Enter between 1 and 2,000 characters for the local demo.".into());
        }
        self.next_id += 1;
        Ok(Event::MessageCreated(Message {
            key: MessageKey {
                channel_id,
                id: 1_000_000_000 + self.next_id,
            },
            author: "You · local echo".into(),
            content: content.into(),
        }))
    }
}
