# Hashing speedtest for varying sizes of data
I am testing SHA-256, SHA512, and BLAKE3 for hashing speed. 

## Results
Tests ran on a 'M1 Pro' Macbook pro with 32GB RAM.

### SHA256:
- 8 bytes: 181430.74 hash/sec, 0.000005512 seconds per hash
- 16 bytes: 187627.49 hash/sec, 0.000005330 seconds per hash
- 32 bytes: 185909.46 hash/sec, 0.000005379 seconds per hash
- 64 bytes: 121828.61 hash/sec, 0.000008208 seconds per hash
- 128 bytes: 91270.45 hash/sec, 0.000010956 seconds per hash
### SHA512:
- 8 bytes: 154678.64 hash/sec, 0.000006465 seconds per hash
- 16 bytes: 153429.85 hash/sec, 0.000006518 seconds per hash
- 32 bytes: 152073.85 hash/sec, 0.000006576 seconds per hash
- 64 bytes: 158023.57 hash/sec, 0.000006328 seconds per hash
- 128 bytes: 110171.49 hash/sec, 0.000009077 seconds per hash
### BLAKE3:
- 8 bytes: 467315.54 hash/sec, 0.000002140 seconds per hash
- 16 bytes: 465933.46 hash/sec, 0.000002146 seconds per hash
- 32 bytes: 465223.65 hash/sec, 0.000002150 seconds per hash
- 64 bytes: 460491.87 hash/sec, 0.000002172 seconds per hash
- 128 bytes: 263540.25 hash/sec, 0.000003794 seconds per hash

## As a Graph

![Graph of Results](imgs/graph_comparison.png)