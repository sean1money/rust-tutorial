use rayon::prelude::*;
use std::{
    sync::atomic::{AtomicU32, Ordering},
    thread,
    time::Duration,
};

use blake3::Hasher;
use tokio::sync::{mpsc, oneshot};

pub const PREFIX_ZERO: &[u8] = &[0, 0, 0];

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (sender, mut receiver) = mpsc::unbounded_channel::<(String, oneshot::Sender<String>)>();
    thread::spawn(move || {
        while let Some((line, reply)) = receiver.blocking_recv() {
            let result = match pow(&line) {
                Some((hash, nonce)) => format!("{}: {}", hash, nonce),
                None => "No solution found".to_string(),
            };
            if let Err(e) = reply.send(result) {
                eprintln!("Failed to send reply: {}", e);
            }
        }
    });

    let iter = AtomicU32::new(0);
    loop {
        if iter.load(Ordering::Relaxed) > 100 {
            break;
        }

        let sender_clone = sender.clone();
        tokio::spawn(async move {
            let (reply, reply_receiver) = oneshot::channel();
            sender_clone.send(("123".to_string(), reply)).unwrap();
            let result = reply_receiver.await.unwrap();
            eprintln!("{}", result);
        });

        tokio::time::sleep(Duration::from_millis(10)).await;
        iter.fetch_add(1, Ordering::Relaxed);
    }

    eprintln!("Task send done");

    thread::sleep(Duration::from_secs(60));
    Ok(())
}

fn pow(s: &str) -> Option<(String, u32)> {
    let hasher = blake3_base_hash(s.as_bytes());
    // rayon here.
    let nonce = (0..u32::MAX).into_par_iter().find_any(|n| {
        let hash = blake3_hash(hasher.clone(), n).as_bytes().to_vec();
        &hash[..PREFIX_ZERO.len()] == PREFIX_ZERO
    });
    nonce.map(|n| {
        let hash = blake3_hash(hasher, &n).to_hex().to_string();
        (hash, n)
    })
}

fn blake3_hash(mut hasher: blake3::Hasher, nonce: &u32) -> blake3::Hash {
    hasher.update(&nonce.to_be_bytes()[..]);
    hasher.finalize()
}

fn blake3_base_hash(data: &[u8]) -> Hasher {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher
}
