# Benchmarking BigInt and IntegerNumber Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a Criterion benchmark suite to compare `num_bigint::BigInt` and `IntegerNumber<BASE>` operations.

**Architecture:** We will configure Cargo to compile a dedicated benchmark target, implement deterministic data generators in the benchmark to construct identical big integers of small (128-bit), medium (2048-bit), and large (32768-bit) sizes, and compare addition, multiplication, and primitive conversions.

**Tech Stack:** Rust (Edition 2024), `criterion` (v0.5), `num-bigint` (v0.4.6).

## Global Constraints
- Do not introduce external randomness dependencies (like `rand`) to ensure benchmarks are stable, deterministic, and compile quickly.
- Compare `IntegerNumber<BASE>` across three bases: `BASE = 256`, `BASE = 4294967296` ($2^{32}$), and `BASE = 18446744073709551616` ($2^{64}$).

---

### Task 1: Configure Cargo.toml for Criterion Benchmarks

**Files:**
- Modify: `Cargo.toml`

**Interfaces:**
- Produces: Criterion dependencies and target registration.

- [ ] **Step 1: Modify Cargo.toml**

Add the `criterion` dependency to `[dev-dependencies]` and add the `[[bench]]` section at the end of the file.

```diff
 [dependencies]
 num-bigint = "0.4.6"
 num-traits = "0.2.19"
 thiserror = "2.0.16"
 num-integer = "0.1.46"
 num-complex = "0.4.6"
 paste = "1.0.15"
 rayon = "1.10.0"
+
+[dev-dependencies]
+criterion = { version = "0.5", features = ["html_reports"] }
+
+[[bench]]
+name = "operations_bench"
+harness = false
```

- [ ] **Step 2: Verify compilation config**

Run: `cargo check`
Expected: Success with no errors.

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml
git commit -m "bench: configure Cargo.toml for criterion benchmarks"
```

---

### Task 2: Implement Input Generator and Benchmark Groups in benches/operations_bench.rs

**Files:**
- Create: `benches/operations_bench.rs`

**Interfaces:**
- Consumes: `mathfn::math::integer_number::IntegerNumber`, `mathfn::math::natural_number::NaturalNumber`, `num_bigint::BigInt`.

- [ ] **Step 1: Write data generator and helper functions**

Create the file `benches/operations_bench.rs` and write the deterministic generator using a simple LCG to match inputs exactly between representations.

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use num_bigint::BigInt;
use num_traits::{Zero, One};
use mathfn::math::integer_number::IntegerNumber;
use mathfn::math::natural_number::NaturalNumber;

// Deterministic generator using LCG: X_{n+1} = (1103515245 * X_n + 12345) & 0x7FFFFFFF
fn generate_digits(count: usize, base: u128) -> Vec<u128> {
    let mut digits = Vec::with_capacity(count);
    let mut state = 42u64;
    for _ in 0..count {
        state = state.wrapping_mul(1103515245).wrapping_add(12345) & 0x7FFFFFFF;
        let mut digit = (state as u128) % base;
        if digits.is_empty() && digit == 0 && base > 1 {
            digit = 1; // Ensure MSB/first digit is non-zero (non-zero magnitude representation)
        }
        digits.push(digit);
    }
    // Reverse because NaturalNumber expects LSD (Least Significant Digit) first
    digits
}

fn to_bigint(digits: &[u128], base: u128) -> BigInt {
    let mut val = BigInt::zero();
    let mut power = BigInt::one();
    let base_big = BigInt::from(base);
    for &d in digits {
        val += BigInt::from(d) * &power;
        power *= &base_big;
    }
    val
}
```

- [ ] **Step 2: Implement addition benchmark function**

Append the `bench_addition` function to `benches/operations_bench.rs`:

```rust
fn bench_addition(c: &mut Criterion) {
    let mut group = c.benchmark_group("Addition");
    
    // Test sizes: (name, base_256_len, base_2_32_len, base_2_64_len)
    let sizes = [
        ("128-bit", 16, 4, 2),
        ("2048-bit", 256, 64, 32),
        ("32768-bit", 4096, 1024, 512),
    ];

    for &(name, len_256, len_2_32, len_2_64) in &sizes {
        // Prepare inputs
        let digits_256_a = generate_digits(len_256, 256);
        let digits_256_b = generate_digits(len_256, 256);
        let a_256 = IntegerNumber::<256>::from(NaturalNumber::<256>::try_from(digits_256_a.clone()).unwrap());
        let b_256 = IntegerNumber::<256>::from(NaturalNumber::<256>::try_from(digits_256_b.clone()).unwrap());
        let a_big = to_bigint(&digits_256_a, 256);
        let b_big = to_bigint(&digits_256_b, 256);

        let digits_32_a = generate_digits(len_2_32, 4294967296);
        let digits_32_b = generate_digits(len_2_32, 4294967296);
        let a_32 = IntegerNumber::<4294967296>::from(NaturalNumber::<4294967296>::try_from(digits_32_a.clone()).unwrap());
        let b_32 = IntegerNumber::<4294967296>::from(NaturalNumber::<4294967296>::try_from(digits_32_b.clone()).unwrap());

        let digits_64_a = generate_digits(len_2_64, 18446744073709551616);
        let digits_64_b = generate_digits(len_2_64, 18446744073709551616);
        let a_64 = IntegerNumber::<18446744073709551616>::from(NaturalNumber::<18446744073709551616>::try_from(digits_64_a.clone()).unwrap());
        let b_64 = IntegerNumber::<18446744073709551616>::from(NaturalNumber::<18446744073709551616>::try_from(digits_64_b.clone()).unwrap());

        group.bench_function(format!("BigInt/{}", name), |b| {
            b.iter(|| black_box(&a_big) + black_box(&b_big))
        });

        group.bench_function(format!("IntegerNumber256/{}", name), |b| {
            b.iter(|| (black_box(a_256.clone()) + black_box(b_256.clone())).unwrap())
        });

        group.bench_function(format!("IntegerNumber2_32/{}", name), |b| {
            b.iter(|| (black_box(a_32.clone()) + black_box(b_32.clone())).unwrap())
        });

        group.bench_function(format!("IntegerNumber2_64/{}", name), |b| {
            b.iter(|| (black_box(a_64.clone()) + black_box(b_64.clone())).unwrap())
        });
    }
    group.finish();
}
```

- [ ] **Step 3: Implement multiplication benchmark function**

Append the `bench_multiplication` function to `benches/operations_bench.rs`:

```rust
fn bench_multiplication(c: &mut Criterion) {
    let mut group = c.benchmark_group("Multiplication");
    
    let sizes = [
        ("128-bit", 16, 4, 2),
        ("2048-bit", 256, 64, 32),
        ("32768-bit", 4096, 1024, 512),
    ];

    for &(name, len_256, len_2_32, len_2_64) in &sizes {
        // Prepare inputs
        let digits_256_a = generate_digits(len_256, 256);
        let digits_256_b = generate_digits(len_256, 256);
        let a_256 = IntegerNumber::<256>::from(NaturalNumber::<256>::try_from(digits_256_a.clone()).unwrap());
        let b_256 = IntegerNumber::<256>::from(NaturalNumber::<256>::try_from(digits_256_b.clone()).unwrap());
        let a_big = to_bigint(&digits_256_a, 256);
        let b_big = to_bigint(&digits_256_b, 256);

        let digits_32_a = generate_digits(len_2_32, 4294967296);
        let digits_32_b = generate_digits(len_2_32, 4294967296);
        let a_32 = IntegerNumber::<4294967296>::from(NaturalNumber::<4294967296>::try_from(digits_32_a.clone()).unwrap());
        let b_32 = IntegerNumber::<4294967296>::from(NaturalNumber::<4294967296>::try_from(digits_32_b.clone()).unwrap());

        let digits_64_a = generate_digits(len_2_64, 18446744073709551616);
        let digits_64_b = generate_digits(len_2_64, 18446744073709551616);
        let a_64 = IntegerNumber::<18446744073709551616>::from(NaturalNumber::<18446744073709551616>::try_from(digits_64_a.clone()).unwrap());
        let b_64 = IntegerNumber::<18446744073709551616>::from(NaturalNumber::<18446744073709551616>::try_from(digits_64_b.clone()).unwrap());

        group.bench_function(format!("BigInt/{}", name), |b| {
            b.iter(|| black_box(&a_big) * black_box(&b_big))
        });

        group.bench_function(format!("IntegerNumber256/{}", name), |b| {
            b.iter(|| (black_box(a_256.clone()) * black_box(b_256.clone())).unwrap())
        });

        group.bench_function(format!("IntegerNumber2_32/{}", name), |b| {
            b.iter(|| (black_box(a_32.clone()) * black_box(b_32.clone())).unwrap())
        });

        group.bench_function(format!("IntegerNumber2_64/{}", name), |b| {
            b.iter(|| (black_box(a_64.clone()) * black_box(b_64.clone())).unwrap())
        });
    }
    group.finish();
}
```

- [ ] **Step 4: Implement conversions benchmark function**

Append the `bench_conversions` function and `criterion_group` macro declarations:

```rust
fn bench_conversions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Conversions");
    
    let val_i128 = -12345678901234567890123456789012345678i128;

    group.bench_function("BigInt/From_i128", |b| {
        b.iter(|| BigInt::from(black_box(val_i128)))
    });

    let bigint_val = BigInt::from(val_i128);
    group.bench_function("BigInt/Into_i128", |b| {
        b.iter(|| i128::try_from(black_box(bigint_val.clone())).unwrap())
    });

    group.bench_function("IntegerNumber256/From_i128", |b| {
        b.iter(|| IntegerNumber::<256>::try_from(black_box(val_i128)).unwrap())
    });

    let int_256 = IntegerNumber::<256>::try_from(val_i128).unwrap();
    group.bench_function("IntegerNumber256/Into_i128", |b| {
        b.iter(|| i128::try_from(black_box(int_256.clone())).unwrap())
    });

    group.bench_function("IntegerNumber2_32/From_i128", |b| {
        b.iter(|| IntegerNumber::<4294967296>::try_from(black_box(val_i128)).unwrap())
    });

    let int_32 = IntegerNumber::<4294967296>::try_from(val_i128).unwrap();
    group.bench_function("IntegerNumber2_32/Into_i128", |b| {
        b.iter(|| i128::try_from(black_box(int_32.clone())).unwrap())
    });

    group.bench_function("IntegerNumber2_64/From_i128", |b| {
        b.iter(|| IntegerNumber::<18446744073709551616>::try_from(black_box(val_i128)).unwrap())
    });

    let int_64 = IntegerNumber::<18446744073709551616>::try_from(val_i128).unwrap();
    group.bench_function("IntegerNumber2_64/Into_i128", |b| {
        b.iter(|| i128::try_from(black_box(int_64.clone())).unwrap())
    });

    group.finish();
}

criterion_group!(benches, bench_addition, bench_multiplication, bench_conversions);
criterion_main!(benches);
```

- [ ] **Step 5: Run compilation check on benches**

Run: `cargo check --benches`
Expected: Success with no errors.

- [ ] **Step 6: Commit new benchmark file**

```bash
git add benches/operations_bench.rs
git commit -m "bench: implement operations_bench.rs"
```
