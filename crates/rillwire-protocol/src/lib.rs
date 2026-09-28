//! Offline parser for a deliberately small documented Gateway subset.
//! No socket, REST client, login, token extraction, or complete session lifecycle.
use rillwire_core::{Message, MessageKey};
use serde::Deserialize;
use serde_json::Value;

pub const MAX_FRAME_BYTES: usize = 1024 * 1024; // local receive budget, NOT Discord's send limit

#[derive(Debug, PartialEq, Eq)]
pub enum Incoming {
    Hello { interval_ms: u64 },
    Ack,
    HeartbeatRequested,
    Reconnect,
    InvalidSession { resumable: bool },
    Message { sequence: u64, message: Message },
    Deleted { sequence: u64, key: MessageKey },
    Unknown { opcode: u64, sequence: Option<u64> },
}

#[derive(Debug)]
pub enum DecodeError {
    TooLarge,
    Malformed,
    InvalidField,
}

#[derive(Deserialize)]
struct Envelope {
    op: u64,
    #[serde(default)]
    d: Value,
    s: Option<u64>,
    t: Option<String>,
}

fn snowflake(value: &Value) -> Result<u64, DecodeError> {
    value
        .as_str()
        .and_then(|s| s.parse::<u64>().ok())
        .ok_or(DecodeError::InvalidField)
}

pub fn decode(bytes: &[u8]) -> Result<Incoming, DecodeError> {
    if bytes.len() > MAX_FRAME_BYTES {
        return Err(DecodeError::TooLarge);
    }
    let e: Envelope = serde_json::from_slice(bytes).map_err(|_| DecodeError::Malformed)?;
    match e.op {
        10 => {
            let interval_ms = e.d["heartbeat_interval"]
                .as_u64()
                .filter(|n| *n > 0)
                .ok_or(DecodeError::InvalidField)?;
            Ok(Incoming::Hello { interval_ms })
        }
        11 => Ok(Incoming::Ack),
        1 => Ok(Incoming::HeartbeatRequested),
        7 => Ok(Incoming::Reconnect),
        9 => Ok(Incoming::InvalidSession {
            resumable: e.d.as_bool().ok_or(DecodeError::InvalidField)?,
        }),
        0 if matches!(e.t.as_deref(), Some("MESSAGE_CREATE" | "MESSAGE_DELETE")) => {
            let sequence = e.s.ok_or(DecodeError::InvalidField)?;
            let key = MessageKey {
                channel_id: snowflake(&e.d["channel_id"])?,
                id: snowflake(&e.d["id"])?,
            };
            if e.t.as_deref() == Some("MESSAGE_DELETE") {
                return Ok(Incoming::Deleted { sequence, key });
            }
            let content = e.d["content"].as_str().ok_or(DecodeError::InvalidField)?;
            let author = e.d["author"]["username"]
                .as_str()
                .ok_or(DecodeError::InvalidField)?;
            Ok(Incoming::Message {
                sequence,
                message: Message {
                    key,
                    author: author.into(),
                    content: content.into(),
                },
            })
        }
        _ => Ok(Incoming::Unknown {
            opcode: e.op,
            sequence: e.s,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn replay_fixture_preserves_large_snowflakes_and_unknown_sequences() {
        let lines = include_str!("../../../fixtures/gateway.ndjson")
            .lines()
            .collect::<Vec<_>>();
        for line in &lines {
            decode(line.as_bytes()).unwrap();
        }
        let Incoming::Message { message, .. } = decode(lines[1].as_bytes()).unwrap() else {
            panic!("message expected")
        };
        assert_eq!(message.key.id, 1_525_000_000_000_000_001);
        assert!(matches!(
            decode(lines[5].as_bytes()).unwrap(),
            Incoming::Unknown {
                sequence: Some(4),
                ..
            }
        ));
    }
    #[test]
    fn malformed_and_oversized_frames_fail_without_panicking() {
        assert!(decode(b"{").is_err());
        assert!(decode(br#"{"op":10,"d":{"heartbeat_interval":0}}"#).is_err());
        assert!(matches!(
            decode(&vec![b' '; MAX_FRAME_BYTES + 1]),
            Err(DecodeError::TooLarge)
        ));
        assert!(
            decode(br#"{"op":0,"s":1,"t":"MESSAGE_CREATE","d":{"id":2,"channel_id":"1"}}"#)
                .is_err()
        );
    }
    #[test]
    fn valid_unrecognized_opcode_is_tolerated() {
        assert!(matches!(
            decode(br#"{"op":999,"d":{}}"#).unwrap(),
            Incoming::Unknown { opcode: 999, .. }
        ));
    }
}
