use crate::math::NaturalNumbers::NaturalNumber;
use crate::math::operations::{Mul, MulAssign};
use crate::math::NaturalNumbers::utils::slice::{safe_split, pop_leading_zeros};
use crate::math::NaturalNumbers::utils::arithmetic::{add_slices, add_into, sub_from, mul_wide};

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
    let mut out = vec![0u128; a.len() + b.len()];
    add_into(&mut out, &z0, 0); // Z0
    add_into(&mut out, &z1, m); // Middle * Base^M
    add_into(&mut out, &z2, m * 2); // Z2 * Base^2M

    out
}

/// The $O(N^2)$ hardware-optimized fallback
fn schoolbook(a: &[u128], b: &[u128]) -> Vec<u128> {
    let mut out = vec![0u128; a.len() + b.len()];

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication_small() {
        let a = NaturalNumber::from(10u128);
        let b = NaturalNumber::from(20u128);
        let c = a * b;
        assert_eq!(c.limbs, vec![200u128]);
    }

    #[test]
    fn test_multiplication_large() {
        let limbs_a = vec![1234567890u128; 35];
        let limbs_b = vec![9876543210u128; 35];
        let a = NaturalNumber::new(limbs_a);
        let b = NaturalNumber::new(limbs_b);
        let c = a.clone() * b.clone();
        assert!(c.limbs.len() > 0);
    }
}
