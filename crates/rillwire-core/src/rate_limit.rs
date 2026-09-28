use std::collections::HashMap;

/// Retry-After gate for a future REST scheduler. Uses monotonic milliseconds.
/// Bucket IDs MUST be combined with major resource IDs (channel/guild/webhook).
/// This handles 429 pauses only; proactive remaining/reset accounting is future work.
#[derive(Default)]
pub struct RetryGate {
    global_until: u64,
    buckets: HashMap<String, u64>,
}

impl RetryGate {
    pub fn defer(
        &mut self,
        bucket_with_major: &str,
        now_ms: u64,
        retry_seconds: f64,
        global: bool,
    ) -> Result<(), &'static str> {
        if !retry_seconds.is_finite()
            || retry_seconds < 0.0
            || retry_seconds > (u64::MAX / 1000) as f64
        {
            return Err("Invalid retry_after");
        }
        self.buckets.retain(|_, until| *until > now_ms);
        let until = now_ms.saturating_add((retry_seconds * 1000.0).ceil() as u64);
        if global {
            self.global_until = self.global_until.max(until);
        } else {
            if self.buckets.len() >= 4096 && !self.buckets.contains_key(bucket_with_major) {
                // Conservatively pause globally instead of growing without bound.
                self.global_until = self.global_until.max(until);
            } else {
                self.buckets
                    .entry(bucket_with_major.into())
                    .and_modify(|v| *v = (*v).max(until))
                    .or_insert(until);
            }
        }
        Ok(())
    }
    pub fn wait_ms(&self, bucket_with_major: &str, now_ms: u64) -> u64 {
        self.global_until
            .max(*self.buckets.get(bucket_with_major).unwrap_or(&0))
            .saturating_sub(now_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fractional_retry_and_global_pause_never_retry_early() {
        let mut gate = RetryGate::default();
        gate.defer("b:channel-1", 100, 0.0001, false).unwrap();
        assert_eq!(gate.wait_ms("b:channel-1", 100), 1);
        assert_eq!(gate.wait_ms("b:channel-2", 100), 0);
        gate.defer("b:channel-2", 100, 1.5, true).unwrap();
        assert_eq!(gate.wait_ms("other", 101), 1499);
        gate.defer("b:channel-1", 101, 0.1, true).unwrap();
        assert_eq!(gate.wait_ms("other", 1599), 1);
        assert_eq!(gate.wait_ms("other", 1600), 0);
    }
    #[test]
    fn malformed_retry_is_rejected() {
        let mut gate = RetryGate::default();
        assert!(gate.defer("b", 0, f64::NAN, false).is_err());
        assert!(gate.defer("b", 0, -1.0, false).is_err());
        assert!(gate.defer("b", 0, f64::INFINITY, false).is_err());
    }
}
