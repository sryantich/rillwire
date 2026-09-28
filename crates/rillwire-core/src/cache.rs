use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MessageKey {
    pub channel_id: u64,
    pub id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub key: MessageKey,
    pub author: String,
    pub content: String,
}

impl Message {
    fn payload_bytes(&self) -> usize {
        self.author.len() + self.content.len()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CacheError {
    Disabled,
    TooLarge,
}

/// Global FIFO retention, capped by entries AND UTF-8 payload bytes.
/// Payload bytes exclude allocator capacity, indexes, and renderer/GPU memory.
/// Existing-message updates preserve insertion order and don't create duplicates.
pub struct MessageCache {
    entries: HashMap<MessageKey, Message>,
    order: VecDeque<MessageKey>,
    max_entries: usize,
    max_payload_bytes: usize,
    payload_bytes: usize,
}

impl MessageCache {
    pub fn new(max_entries: usize, max_payload_bytes: usize) -> Self {
        Self {
            entries: HashMap::new(),
            order: VecDeque::new(),
            max_entries,
            max_payload_bytes,
            payload_bytes: 0,
        }
    }

    pub fn insert(&mut self, mut message: Message) -> Result<(), CacheError> {
        if self.max_entries == 0 || self.max_payload_bytes == 0 {
            return Err(CacheError::Disabled);
        }
        let size = message.payload_bytes();
        if size > self.max_payload_bytes {
            return Err(CacheError::TooLarge);
        }
        // Avoid retaining a caller's oversized String reservation.
        message.author.shrink_to_fit();
        message.content.shrink_to_fit();
        let key = message.key;
        if let Some(previous) = self.entries.insert(key, message) {
            self.payload_bytes -= previous.payload_bytes();
        } else {
            self.order.push_back(key);
        }
        self.payload_bytes += size;
        while self.entries.len() > self.max_entries || self.payload_bytes > self.max_payload_bytes {
            if let Some(oldest) = self.order.pop_front()
                && let Some(removed) = self.entries.remove(&oldest)
            {
                self.payload_bytes -= removed.payload_bytes();
            }
        }
        Ok(())
    }

    pub fn remove(&mut self, key: MessageKey) -> bool {
        if let Some(message) = self.entries.remove(&key) {
            self.payload_bytes -= message.payload_bytes();
            self.order.retain(|item| *item != key);
            true
        } else {
            false
        }
    }

    pub fn get(&self, key: MessageKey) -> Option<&Message> {
        self.entries.get(&key)
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn payload_bytes(&self) -> usize {
        self.payload_bytes
    }
    pub fn channel_keys(&self, channel_id: u64) -> Vec<MessageKey> {
        self.order
            .iter()
            .copied()
            .filter(|key| key.channel_id == channel_id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn msg(id: u64, channel: u64, body: &str) -> Message {
        Message {
            key: MessageKey {
                id,
                channel_id: channel,
            },
            author: "a".into(),
            content: body.into(),
        }
    }
    #[test]
    fn global_budget_evicts_across_channels_and_deduplicates() {
        let mut cache = MessageCache::new(2, 100);
        cache.insert(msg(1, 1, "first")).unwrap();
        cache.insert(msg(2, 2, "second")).unwrap();
        cache.insert(msg(2, 2, "edited")).unwrap();
        cache.insert(msg(3, 3, "third")).unwrap();
        assert_eq!(cache.len(), 2);
        assert!(cache.channel_keys(1).is_empty());
        assert_eq!(cache.channel_keys(2).len(), 1);
        assert_eq!(cache.payload_bytes(), 13);
    }
    #[test]
    fn oversized_edit_does_not_destroy_existing_message() {
        let mut cache = MessageCache::new(3, 8);
        cache.insert(msg(1, 1, "old")).unwrap();
        assert_eq!(
            cache.insert(msg(1, 1, "much too large")),
            Err(CacheError::TooLarge)
        );
        assert_eq!(
            cache
                .get(MessageKey {
                    id: 1,
                    channel_id: 1
                })
                .unwrap()
                .content,
            "old"
        );
    }
    #[test]
    fn utf8_byte_budget_and_delete_accounting() {
        let mut cache = MessageCache::new(20, 8);
        cache.insert(msg(1, 1, "界界")).unwrap();
        cache.insert(msg(2, 1, "ok")).unwrap();
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.payload_bytes(), 3);
        assert!(cache.remove(MessageKey {
            id: 2,
            channel_id: 1
        }));
        assert_eq!(cache.payload_bytes(), 0);
        assert!(cache.is_empty());
    }
    #[test]
    fn zero_capacity_disables_retention() {
        assert_eq!(
            MessageCache::new(0, 8).insert(msg(1, 1, "x")),
            Err(CacheError::Disabled)
        );
    }
}
