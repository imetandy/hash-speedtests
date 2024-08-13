use sha2::{Sha256, Sha512, Digest};
use blake3::Hasher;
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

fn sha256_measure_hash_speed(data: &[u8], label: &str) {
    let iterations = 1_000_000;
    let start = Instant::now();

    for _ in 0..iterations {
        sha256_hash_data(data);
    }

    let duration = start.elapsed();
    let duration_per_hash = duration.as_secs_f64() / iterations as f64;

    println!("{} bytes: {:.9} seconds per hash", label, duration_per_hash);
}


fn sha512_measure_hash_speed(data: &[u8], label: &str) {
    let iterations = 1_000_000;
    let start = Instant::now();

    for _ in 0..iterations {
        sha512_hash_data(data);
    }

    let duration = start.elapsed();
    let duration_per_hash = duration.as_secs_f64() / iterations as f64;

    println!("{} bytes: {:.9} seconds per hash", label, duration_per_hash);
}

fn blake3_measure_hash_speed(data: &[u8], label: &str) {
    let iterations = 1_000_000;
    let start = Instant::now();

    for _ in 0..iterations {
        blake3_hash_data(data);
    }

    let duration = start.elapsed();
    let duration_per_hash = duration.as_secs_f64() / iterations as f64;

    println!("{} bytes: {:.9} seconds per hash", label, duration_per_hash);
}


fn main() {
    let data_8 = [0u8; 8];
    let data_32 = [0u8; 32];
    let data_64 = [0u8; 64];
    let data_128 = [0u8; 128];
    println!("SHA256:");
    sha256_measure_hash_speed(&data_8, "8");
    sha256_measure_hash_speed(&data_32, "32");
    sha256_measure_hash_speed(&data_64, "64");
    sha256_measure_hash_speed(&data_128, "128");
    println!("SHA512:");
    sha512_measure_hash_speed(&data_8, "8");
    sha512_measure_hash_speed(&data_32, "32");
    sha512_measure_hash_speed(&data_64, "64");
    sha512_measure_hash_speed(&data_128, "128");
    println!("BLAKE3:");
    blake3_measure_hash_speed(&data_8, "8");
    blake3_measure_hash_speed(&data_32, "32");
    blake3_measure_hash_speed(&data_64, "64");
    blake3_measure_hash_speed(&data_128, "128");
}