//! Deterministic heartbeat timing primitive, not a complete Gateway session manager.
#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Wait,
    Send,
    Reconnect,
}

pub struct Heartbeat {
    interval_ms: u64,
    next_due_ms: u64,
    awaiting_ack: bool,
    failed: bool,
}

impl Heartbeat {
    /// The caller supplies randomized first-delay in [0, interval).
    pub fn new(now_ms: u64, interval_ms: u64, first_delay_ms: u64) -> Result<Self, &'static str> {
        if interval_ms == 0 || first_delay_ms >= interval_ms {
            return Err("Invalid heartbeat interval or jitter delay");
        }
        Ok(Self {
            interval_ms,
            next_due_ms: now_ms.saturating_add(first_delay_ms),
            awaiting_ack: false,
            failed: false,
        })
    }
    pub fn poll(&mut self, now_ms: u64) -> Action {
        if self.failed {
            return Action::Reconnect;
        }
        if now_ms < self.next_due_ms {
            return Action::Wait;
        }
        if self.awaiting_ack {
            self.failed = true;
            return Action::Reconnect;
        }
        self.awaiting_ack = true;
        self.next_due_ms = now_ms.saturating_add(self.interval_ms);
        Action::Send
    }
    pub fn acknowledge(&mut self) {
        self.awaiting_ack = false;
    }
    /// Opcode 1 requests an immediate heartbeat without moving the periodic deadline.
    pub fn server_request(&mut self) -> Action {
        if self.failed {
            return Action::Reconnect;
        }
        self.awaiting_ack = true;
        Action::Send
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_ack_requires_reconnect_even_if_ack_arrives_late() {
        let mut h = Heartbeat::new(0, 1000, 50).unwrap();
        assert_eq!(h.poll(49), Action::Wait);
        assert_eq!(h.poll(50), Action::Send);
        assert_eq!(h.poll(1050), Action::Reconnect);
        h.acknowledge();
        assert_eq!(h.poll(2050), Action::Reconnect);
    }
    #[test]
    fn ack_and_server_requested_heartbeat() {
        let mut h = Heartbeat::new(0, 1000, 0).unwrap();
        assert_eq!(h.poll(0), Action::Send);
        h.acknowledge();
        assert_eq!(h.server_request(), Action::Send);
        h.acknowledge();
        assert_eq!(h.poll(999), Action::Wait);
        assert_eq!(h.poll(1000), Action::Send);
    }
    #[test]
    fn invalid_intervals_are_rejected() {
        assert!(Heartbeat::new(0, 0, 0).is_err());
        assert!(Heartbeat::new(0, 10, 10).is_err());
    }
}
