use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::{Mul, MulAssign};

// The threshold where the overhead of Karatsuba becomes worse than Schoolbook
const KARATSUBA_CUTOFF: usize = 30;

impl MulAssign for NaturalNumber {
    fn mul_assign(&mut self, other: NaturalNumber) {
        // 1. Calculate the product into a new vector using our optimized algorithm
        let mut result_limbs = karatsuba(&self.limbs, &other.limbs);

        // 2. Strip any trailing zeros (leading zeros in BigInt logic)
        pop_leading_zeros(&mut result_limbs);

        // 3. Swap the old vector out for the new one.
        // This is incredibly fast—it just updates the pointer, length, and capacity
        // without copying the actual data in memory.
        self.limbs = result_limbs;
    }
}

// Now we can update `Mul` to just call `MulAssign`, keeping your code perfectly DRY!
impl Mul for NaturalNumber {
    type Output = NaturalNumber;

    fn mul(mut self, other: NaturalNumber) -> NaturalNumber {
        // This directly calls the `mul_assign` function above
        self *= other;
        self
    }
}

/// The Recursive Divide and Conquer Algorithm
fn karatsuba(a: &[u128], b: &[u128]) -> Vec<u128> {
    // 1. Base Case: If arrays are small, let the CPU hardware take over
    if a.len() <= KARATSUBA_CUTOFF || b.len() <= KARATSUBA_CUTOFF {
        return schoolbook(a, b);
    }

    // Calculate the halfway point (M)
    let m = a.len().max(b.len()) / 2;

    // 2. Split the slices in half (Zero memory allocations here, just view pointers!)
    let (a_lo, a_hi) = safe_split(a, m);
    let (b_lo, b_hi) = safe_split(b, m);

    // 3. Calculate Z0 and Z2 recursively
    let z0 = karatsuba(a_lo, b_lo);
    let z2 = karatsuba(a_hi, b_hi);

    // 4. Calculate Z1: (a_lo + a_hi) * (b_lo + b_hi)
    let a_sum = add_slices(a_lo, a_hi);
    let b_sum = add_slices(b_lo, b_hi);
    let mut z1 = karatsuba(&a_sum, &b_sum);

    // 5. The Magic Algebra Step: Isolate the middle term
    // middle = z1 - z0 - z2
    sub_from(&mut z1, &z0);
    sub_from(&mut z1, &z2);

    // 6. Assemble the final answer: Z0 + (Middle << M) + (Z2 << 2M)
    // We achieve the bit-shift (<< M) by simply placing the arrays M indices higher!
    let mut out = vec![0; a.len() + b.len()];
    add_into(&mut out, &z0, 0); // Z0
    add_into(&mut out, &z1, m); // Middle * Base^M
    add_into(&mut out, &z2, m * 2); // Z2 * Base^2M

    out
}

/// The $O(N^2)$ hardware-optimized fallback
fn schoolbook(a: &[u128], b: &[u128]) -> Vec<u128> {
    let mut out = vec![0; a.len() + b.len()];

    for (i, &a_val) in a.iter().enumerate() {
        let mut carry = 0u128;
        for (j, &b_val) in b.iter().enumerate() {
            // Get the 256-bit product split into two 128-bit chunks
            let (prod_hi, prod_lo) = mul_wide(a_val, b_val);

            // Add lower part of product + carry + existing value in array
            let (sum1, over1) = out[i + j].overflowing_add(prod_lo);
            let (sum2, over2) = sum1.overflowing_add(carry);

            out[i + j] = sum2;

            // The new carry is the upper part of the product + any overflow from addition
            carry = prod_hi + (over1 as u128) + (over2 as u128);
        }
        out[i + b.len()] = carry;
    }
    out
}

// =======================================================================
// Helper Math Operations for Slices
// =======================================================================

/// Safely splits a slice even if it is shorter than M
fn safe_split(slice: &[u128], m: usize) -> (&[u128], &[u128]) {
    match m >= slice.len() {
        true => (slice, &[]),
        false => slice.split_at(m),
    }
}

/// Adds two slices together and returns a new Vec
fn add_slices(a: &[u128], b: &[u128]) -> Vec<u128> {
    let max_len = a.len().max(b.len());
    let mut out = vec![0; max_len];
    let mut carry = 0u128;

    for i in 0..max_len {
        let a_val = a.get(i).copied().unwrap_or(0);
        let b_val = b.get(i).copied().unwrap_or(0);
        let (sum1, over1) = a_val.overflowing_add(b_val);
        let (sum2, over2) = sum1.overflowing_add(carry);
        out[i] = sum2;
        carry = (over1 as u128) + (over2 as u128);
    }
    if carry > 0 {
        out.push(carry);
    }
    out
}

/// Removes trailing zeros from a vector
fn pop_leading_zeros(slice: &mut Vec<u128>) {
    while slice.last() == Some(&0) && slice.len() > 1 {
        slice.pop();
    }
}

/// Adds a slice into an existing mutable array starting at a specific index offset
fn add_into(target: &mut [u128], addend: &[u128], offset: usize) {
    let mut carry = 0u128;
    for (i, &val) in addend.iter().enumerate() {
        if offset + i >= target.len() {
            break;
        }

        let (sum1, over1) = target[offset + i].overflowing_add(val);
        let (sum2, over2) = sum1.overflowing_add(carry);

        target[offset + i] = sum2;
        carry = (over1 as u128) + (over2 as u128);
    }
    // Propagate remaining carry
    let mut idx = offset + addend.len();
    while carry > 0 && idx < target.len() {
        let (sum, over) = target[idx].overflowing_add(carry);
        target[idx] = sum;
        carry = over as u128;
        idx += 1;
    }
}

/// In-place subtraction (target = target - sub)
fn sub_from(target: &mut [u128], sub: &[u128]) {
    let mut borrow = 0u128;
    for i in 0..target.len() {
        let sub_val = sub.get(i).copied().unwrap_or(0);

        let (diff1, under1) = target[i].overflowing_sub(sub_val);
        let (diff2, under2) = diff1.overflowing_sub(borrow);

        target[i] = diff2;
        borrow = (under1 as u128) + (under2 as u128);
    }
}

/// Simulates a 256-bit multiplication returning (high_128, low_128)
fn mul_wide(a: u128, b: u128) -> (u128, u128) {
    // Break 128-bit numbers into 64-bit halves
    let a_lo = (a as u64) as u128;
    let a_hi = a >> 64;
    let b_lo = (b as u64) as u128;
    let b_hi = b >> 64;

    // Cross multiply
    let t0 = a_lo * b_lo;
    let t1 = a_lo * b_hi;
    let t2 = a_hi * b_lo;
    let t3 = a_hi * b_hi;

    // Resolve overlaps
    let (mid, over1) = t1.overflowing_add(t2);
    let (mid_lo, over2) = mid.overflowing_add(t0 >> 64);

    let lo = (t0 & 0xFFFFFFFFFFFFFFFF) | (mid_lo << 64);
    let hi = t3 + ((over1 as u128) << 64) + (mid >> 64) + (over2 as u128);

    (hi, lo)
}
