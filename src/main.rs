use blake3::Hasher;
use sha2::{Digest, Sha256, Sha512};
use std::time::Instant;

fn sha256_hash_data(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn sha512_hash_data(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn blake3_hash_data(data: &[u8]) -> Vec<u8> {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize().as_bytes().to_vec()
}

/** Generic function to measure hash speed. It accepts a closure that hashes the data */
fn measure_hash_speed<F>(data: &[u8], label: &str, hash_fn: F, algorithm: &str)
where
    F: Fn(&[u8]) -> Vec<u8>,
{
    let iterations = 1_000_000;
    let start = Instant::now();

    for _ in 0..iterations {
        hash_fn(data);
    }

    let duration = start.elapsed();
    let duration_per_hash = duration.as_secs_f64() / iterations as f64;
    let hashes_per_second = 1.0 / duration_per_hash;

    println!(
        "{} - {} bytes: {:.2} hash/sec, {:.9} seconds per hash",
        algorithm, label, hashes_per_second, duration_per_hash
    );
}

fn main() {
    let data_sizes = [8, 16, 32, 64, 128];

    let hash_functions: [(&str, &dyn Fn(&[u8]) -> Vec<u8>); 3] = [
        ("SHA256", &sha256_hash_data),
        ("SHA512", &sha512_hash_data),
        ("BLAKE3", &blake3_hash_data),
    ];

    for (algorithm, hash_function) in &hash_functions {
        println!("\n{}:", algorithm);
        for &size in &data_sizes {
            let data = vec![0u8; size];
            measure_hash_speed(&data, &size.to_string(), *hash_function, algorithm);
        }
    }
}
