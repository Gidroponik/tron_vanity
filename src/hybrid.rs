use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha3::{Keccak256, Digest};
use sha2::Sha256;
use std::fs::OpenOptions;
use std::io::Write as IoWrite;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use rayon::prelude::*;
use rand::Rng;

const BATCH_SIZE: usize = 100_000; // You Can Change

pub struct CpuGenerator {
    checker: crate::PatternChecker,
    secp: Secp256k1<secp256k1::All>,
}

impl CpuGenerator {
    pub fn new(
        checker: crate::PatternChecker,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            checker,
            secp: Secp256k1::new(),
        })
    }

    pub fn generate_keypairs_cpu(&self, batch_size: usize) -> Vec<([u8; 32], Vec<u8>)> {
        (0..batch_size)
            .into_par_iter()
            .filter_map(|_| {
                // Генерируем случайный приватный ключ
                let mut rng = rand::thread_rng();
                let private_key_bytes: [u8; 32] = rng.gen();

                let secret_key = SecretKey::from_slice(&private_key_bytes).ok()?;
                let public_key = PublicKey::from_secret_key(&self.secp, &secret_key);
                let pubkey_bytes = public_key.serialize_uncompressed().to_vec();

                Some((private_key_bytes, pubkey_bytes))
            })
            .collect()
    }

    pub fn pubkey_to_address(&self, pubkey: &[u8]) -> String {
        let mut hasher = Keccak256::new();
        hasher.update(&pubkey[1..]);
        let hash = hasher.finalize();

        let mut address_bytes = [0u8; 21];
        address_bytes[0] = 0x41;
        address_bytes[1..21].copy_from_slice(&hash[12..32]);

        let mut hasher1 = Sha256::new();
        hasher1.update(&address_bytes);
        let hash1 = hasher1.finalize();

        let mut hasher2 = Sha256::new();
        hasher2.update(&hash1);
        let hash2 = hasher2.finalize();

        let mut final_bytes = [0u8; 25];
        final_bytes[..21].copy_from_slice(&address_bytes);
        final_bytes[21..25].copy_from_slice(&hash2[..4]);

        bs58::encode(final_bytes).into_string()
    }

    pub fn run(&self, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let total_attempts = Arc::new(AtomicU64::new(0));
        let found_count = Arc::new(AtomicU64::new(0));
        let start_time = std::time::Instant::now();

        let total_attempts_stats = Arc::clone(&total_attempts);
        let found_count_stats = Arc::clone(&found_count);
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(2));
                let attempts = total_attempts_stats.load(Ordering::Relaxed);
                let found = found_count_stats.load(Ordering::Relaxed);
                let elapsed = start_time.elapsed().as_secs_f64();
                let rate = if elapsed > 0.0 { (attempts as f64 / elapsed) as u64 } else { 0 };
                print!("\r{} addr/s | {} attempts | {} found", rate, attempts, found);
                std::io::stdout().flush().unwrap();
            }
        });

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(output_file)?;

        let running = Arc::new(AtomicU64::new(1));
        let running_signal = Arc::clone(&running);
        ctrlc::set_handler(move || {
            running_signal.store(0, Ordering::Relaxed);
        })?;

        while running.load(Ordering::Relaxed) == 1 {
            // Генерируем приватные и публичные ключи на CPU параллельно
            let batch_start = std::time::Instant::now();
            let keypairs = self.generate_keypairs_cpu(BATCH_SIZE);
            let gen_time = batch_start.elapsed();

            if keypairs.is_empty() {
                continue;
            }

            let check_start = std::time::Instant::now();
            let matches: Vec<_> = keypairs
                .par_iter()
                .filter_map(|(priv_key, pubkey)| {
                    let address = self.pubkey_to_address(pubkey);
                    if let Some(pattern) = self.checker.check(&address) {
                        Some((address, pattern, *priv_key))
                    } else {
                        None
                    }
                })
                .collect();
            let check_time = check_start.elapsed();

            total_attempts.fetch_add(keypairs.len() as u64, Ordering::Relaxed);

            static BATCH_COUNTER: AtomicU64 = AtomicU64::new(0);
            if BATCH_COUNTER.fetch_add(1, Ordering::Relaxed) % 10 == 0 {
                println!("\n[DEBUG] Gen: {:.2}s | Check: {:.2}s | Batch: {} keys",
                    gen_time.as_secs_f64(), check_time.as_secs_f64(), keypairs.len());
            }

            for (address, pattern, priv_key) in matches {
                if running.load(Ordering::Relaxed) == 0 {
                    break;
                }

                let count = found_count.fetch_add(1, Ordering::Relaxed) + 1;
                let priv_key_hex = hex::encode(priv_key);

                let line = format!("{} | {} | {}\n", address, pattern, priv_key_hex);
                file.write_all(line.as_bytes())?;
                file.flush()?;

                println!("\n[{}] {} | {} | {}", count, address, pattern, priv_key_hex);
            }
        }

        println!("\n\nStopped.");
        let total = total_attempts.load(Ordering::Relaxed);
        let found = found_count.load(Ordering::Relaxed);
        let elapsed = start_time.elapsed().as_secs_f64();
        let rate = if elapsed > 0.0 { (total as f64 / elapsed) as u64 } else { 0 };

        println!("Total: {} attempts | {} found | {} addr/s", total, found, rate);

        Ok(())
    }
}
