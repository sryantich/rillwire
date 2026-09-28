use rillwire_core::{Message, MessageCache, MessageKey};
use rillwire_protocol::Incoming;
use serde_json::json;
use std::{hint::black_box, time::Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str).unwrap_or("replay") {
        "replay" => {
            let mut cache = MessageCache::new(4096, 4 * 1024 * 1024);
            let mut last_sequence = None;
            let mut count = 0;
            for line in include_str!("../../../fixtures/gateway.ndjson").lines() {
                match rillwire_protocol::decode(line.as_bytes())
                    .map_err(|e| format!("decode: {e:?}"))?
                {
                    Incoming::Message { sequence, message } => {
                        last_sequence = Some(sequence);
                        cache.insert(message).map_err(|e| format!("cache: {e:?}"))?;
                    }
                    Incoming::Deleted { sequence, key } => {
                        last_sequence = Some(sequence);
                        cache.remove(key);
                    }
                    Incoming::Unknown {
                        sequence: Some(s), ..
                    } => last_sequence = Some(s),
                    _ => (),
                }
                count += 1;
            }
            println!(
                "{}",
                json!({"mode":"synthetic-offline-replay","frames":count,"retained_messages":cache.len(),"last_sequence":last_sequence,"payload_bytes":cache.payload_bytes()})
            );
        }
        "bench" => {
            let count = args
                .get(2)
                .map(|s| s.parse::<u64>())
                .transpose()?
                .unwrap_or(100_000);
            if !(1..=10_000_000).contains(&count) {
                return Err("count must be 1..=10000000".into());
            }
            let mut samples = Vec::new();
            let mut retained = 0;
            let mut payload_bytes = 0;
            for run in 0..6 {
                let mut cache = MessageCache::new(4096, 4 * 1024 * 1024);
                let start = Instant::now();
                for id in 0..count {
                    cache
                        .insert(Message {
                            key: MessageKey {
                                channel_id: id % 8,
                                id,
                            },
                            author: "Synthetic author".into(),
                            content: format!(
                                "Synthetic message {id}: bounded retention under load."
                            ),
                        })
                        .unwrap();
                }
                black_box(&cache);
                if run > 0 {
                    samples.push(start.elapsed().as_secs_f64() * 1000.0);
                }
                retained = cache.len();
                payload_bytes = cache.payload_bytes();
            }
            samples.sort_by(f64::total_cmp);
            println!(
                "{}",
                json!({"mode":"cache-insert-microbenchmark","platform":std::env::consts::OS,"arch":std::env::consts::ARCH,"build":if cfg!(debug_assertions){"debug"}else{"release"},"messages_per_run":count,"runs":5,"warmup_runs":1,"median_ms":samples[2],"samples_ms":samples,"retained_messages":retained,"utf8_payload_bytes":payload_bytes,"note":"Includes message creation and insertion. Excludes UI, process RSS, GPU, network, and Discord. Not a client comparison."})
            );
        }
        _ => return Err("Usage: rillwire-lab [replay | bench [count]]".into()),
    }
    Ok(())
}
