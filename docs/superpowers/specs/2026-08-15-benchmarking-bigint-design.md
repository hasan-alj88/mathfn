# Design Specification: Benchmarking BigInt and IntegerNumber

This specification details the plan to benchmark the custom `IntegerNumber<BASE>` representation against the standard library's `num-bigint::BigInt` implementation across various operations, input sizes, and digit bases.

## Goal Description
Understand the performance characteristics, overheads, and scaling properties of `IntegerNumber` compared to `num-bigint::BigInt` using the `criterion` benchmarking library.

Specifically, we want to:
- Compare `IntegerNumber<BASE>` with `BASE = 256` (8-bit), `BASE = 4,294,967,296` (32-bit), and `BASE = 18,446,744,073,709,551,616` (64-bit).
- Test operations: Addition (`std::ops::Add`), Multiplication (`std::ops::Mul`), and primitive conversions (to/from `i128`).
- Test multiple input sizes: Small (128-bit), Medium (2048-bit), and Large (32768-bit).

## Proposed Changes

### Cargo.toml
Add `criterion` under `[dev-dependencies]` and register `operations_bench` as a benchmark target without the standard test harness.

### Benchmark file: `benches/operations_bench.rs`
Create a new file containing:
1. Deterministic data generation logic to produce identical numbers of target bit lengths for both representation types.
2. Criterion benchmark groups for **Addition**, **Multiplication**, and **Conversions**.
3. Comparisons between:
   - `num_bigint::BigInt`
   - `IntegerNumber<256>`
   - `IntegerNumber<4294967296>`
   - `IntegerNumber<18446744073709551616>`

---

## Component Details

### 1. Target Bit Sizes & Digit Counts
For each of the three bases, the input sizes will map to the following digit counts:

| Target Size | Digit Count (Base 256) | Digit Count (Base $2^{32}$) | Digit Count (Base $2^{64}$) |
|---|---|---|---|
| **Small (128-bit)** | 16 | 4 | 2 |
| **Medium (2048-bit)** | 256 | 64 | 32 |
| **Large (32768-bit)** | 4096 | 1024 | 512 |

### 2. Input Data Generator
We will use a deterministic pseudo-random number generator (specifically a simple linear congruential generator) inside the benchmark file to construct matching inputs for:
- `IntegerNumber<BASE>`
- `BigInt`

### 3. Benchmark Structure
We will use Criterion's parameterized benchmark groups to compare the execution time of:
- `add`
- `mul`
- `try_from` / `try_into` conversions

---

## Verification Plan

### Automated Tests & Benchmarks
- Run `cargo check --benches` to ensure the benchmark code compiles.
- Run `cargo bench -- --quick` or a short run of the benchmark to verify execution.
