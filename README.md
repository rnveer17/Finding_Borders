# Finding Borders – Two Algorithms in Rust

## Overview

Solves the Finding Borders problem using two different approaches:
1. **Naive Border Check** – simple slice comparison.
2. **KMP Border Compute** – uses the prefix function for optimal performance.

The goal is to find all border lengths (prefix that is also a suffix) of a given string.

---

## Algorithms

### 1. Naive Border Check (`naive_border_check`)

Checks for every possible length from 1 to n-1. For each length, it compares the prefix and suffix slices directly.

**Time:** O(n²) – each comparison can take O(n) in the worst case.  
**Space:** O(1) – only a few integers.

### 2. KMP Border Compute (`kmp_border_compute`)

Computes the prefix function π, which stores the longest proper border ending at each position. Then follows π[n-1] to collect all border lengths.

**Time:** O(n) – one pass to compute π, plus O(number of borders).  
**Space:** O(n) – stores the π array.

---

## Benchmark

The benchmark runs both algorithms on **all CSES test cases**. It reads each `.in` file from `tests/inout/`, measures execution time for both algorithms, and prints a comparison table.


```bash
cargo run --release --bin bench
```

**Results (n = 1,000,000):**

| n | Naive (ms) | KMP (ms) |
|---|------------|----------|
| 1,000,000 | N/A | 10.92 |
| 1,000,000 | N/A | 17.46 |

The naive algorithm is skipped for large inputs (marked as N/A) because its O(n²) complexity makes it impractical. KMP runs in milliseconds, easily meeting the time limit.

---

## Why KMP is Faster

### 1. Algorithmic Complexity

- **Naive**  re‑compares substrings from scratch for every candidate length, leading to O(n²). This is why it's skipped for large inputs. 
- **KMP** reuses previous matching information via the prefix function, achieving O(n) and running in milliseconds even for `n = 10⁶`.

### 2. Memory Access & Cache

- KMP uses a single `Vec<usize>` for the prefix array – contiguous memory, cache‑friendly.
- The naive approach would cause many random memory accesses, making it much slower.

### 3. Real‑world Performance

The benchmark shows KMP runs in under 20 ms for `n = 1,000,000` – well within the 1‑second limit. The naive version would take seconds or minutes, so it is only used for testing small strings.

**Conclusion:** KMP is the optimal choice for this problem. The naive algorithm is included for demonstration and correctness verification on small inputs.

---

## How to Run

### Run the main program
```bash
cargo run
```

### Run the benchmark
```bash
cargo run --release --bin bench
```

### Run tests
```bash
cargo test
```
