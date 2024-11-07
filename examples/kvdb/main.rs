use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use alloy::primitives::{keccak256, B256};
use anyhow::Result;
use kvdb_rocksdb::{Database, DatabaseConfig};
use secp256k1::rand::Rng;
use sysinfo::{get_current_pid, System};

#[derive(Clone)]
struct KeyValue {
    key: B256,
    val: B256,
}

fn next(seed: B256) -> B256 {
    let mut buf = [0u8; 33];
    buf[0..32].copy_from_slice(&seed[..]);
    buf[32] = 1;

    keccak256(&buf[..])
}

impl KeyValue {
    fn with_seed(seed: B256) -> Self {
        KeyValue {
            key: next(seed),
            val: next(next(seed)),
        }
    }

    fn new() -> Self {
        let mut rng = rand_core::OsRng;
        let seed = rng.gen::<[u8; 32]>();

        let seed = B256::from(seed);
        Self::with_seed(seed)
    }
}

impl Iterator for KeyValue {
    type Item = (B256, B256);

    fn next(&mut self) -> Option<Self::Item> {
        let result = (self.key, self.val);
        self.key = next(self.key);
        self.val = next(self.val);

        Some(result)
    }
}

fn proc_memory_usage() -> u64 {
    let mut sys = System::new();
    let self_pid = get_current_pid().ok();
    let memory = if let Some(self_pid) = self_pid {
        if sys.refresh_process(self_pid) {
            let proc = sys
                .process(self_pid)
                .expect("Above refresh_process succeeds, this should be Some(), qed");
            proc.memory()
        } else {
            0
        }
    } else {
        0
    };

    memory
}

fn main() -> Result<()> {
    // MB per column family
    let mb_per_col = std::env::args()
        .nth(1)
        .map(|arg| {
            arg.parse()
                .expect("Megabytes per col - should be integer or missing")
        })
        .unwrap_or(1);

    let exit = Arc::new(AtomicBool::new(false));
    let ctrlc_exit = exit.clone();

    ctrlc::set_handler(move || {
        println!("\nRemoving temp database...\n");
        ctrlc_exit.store(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");

    let column_count = 100;
    let mut config = DatabaseConfig::with_columns(column_count);
    for c in 0..=column_count {
        config.memory_budget.insert(c, mb_per_col);
    }
    let dir = tempfile::Builder::new()
        .prefix("rocksdb-example")
        .tempdir()
        .unwrap();

    println!(
        "Database is put in: {} (maybe check if it was deleted)",
        dir.path().to_string_lossy()
    );

    let db = Database::open(&config, &dir.path())?;

    let mut step = 0;
    let mut keyvalues = KeyValue::new();
    while !exit.load(Ordering::Relaxed) {
        let col = step % 100;

        let key_values: Vec<(B256, B256)> = keyvalues.clone().take(128).collect();
        let mut transaction = db.transaction();
        for (k, v) in key_values.iter() {
            transaction.put(col, k.as_ref(), v.as_ref());
        }
        db.write(transaction).expect("writing failed");

        let mut seed = B256::ZERO;
        for (k, _) in key_values.iter() {
            let mut buf = [0u8; 64];
            buf[0..32].copy_from_slice(seed.as_ref());
            let val = db
                .get(col, k.as_ref())
                .expect("Db fail")
                .expect("Was put above");
            buf[32..64].copy_from_slice(val.as_ref());

            seed = keccak256(&buf[..]);
        }

        let mut transaction = db.transaction();
        // delete all but one to avoid too much bloating
        for (k, _) in key_values.iter().take(127) {
            transaction.delete(col, k.as_ref());
        }
        db.write(transaction).expect("delete failed");

        keyvalues = KeyValue::with_seed(seed);

        if step % 10000 == 9999 {
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");

            println!("{}", timestamp);
            println!(
                "\tData written: {} keys - {} Mb",
                step + 1,
                ((step + 1) * 64 * 128) / 1024 / 1024
            );
            println!(
                "\tProcess memory used as seen by the OS: {} Mb",
                proc_memory_usage() / 1024
            );
        }

        step += 1;
    }

    Ok(())
}
