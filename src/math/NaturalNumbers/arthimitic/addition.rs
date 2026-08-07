use crate::math::NaturalNumbers::NaturalNumber;
use rayon::prelude::*;
use std::ops::{Add, AddAssign};

impl AddAssign for NaturalNumber {
    fn add_assign(&mut self, other: NaturalNumber) {
        let mut max_len = self.limbs.len().max(other.limbs.len());

        // Mutate self in-place
        self.limbs.resize(max_len, 0);

        let mut carry_limbs = other.limbs;
        carry_limbs.resize(max_len, 0);

        let mut has_carries = true;

        while has_carries {
            // 1. Parallel Addition
            let overflow_bits: Vec<u128> = self
                .limbs
                .par_iter_mut()
                .zip(carry_limbs.par_iter())
                .map(|(res, carry)| {
                    let (sum, overflow) = res.overflowing_add(*carry);
                    *res = sum;
                    match overflow {
                        true => 1,
                        false => 0,
                    }
                })
                .collect();

            // 2. Shift the carries left by 1 index
            carry_limbs[0] = 0;

            carry_limbs[1..]
                .par_iter_mut()
                .zip(overflow_bits[..max_len - 1].par_iter())
                .for_each(|(target, &bit)| {
                    *target = bit;
                });

            // 3. Handle the most significant carry out using `match`
            match overflow_bits[max_len - 1] {
                1 => {
                    self.limbs.push(0); // Expand the result vector
                    carry_limbs.push(1); // Place the shifted carry
                    max_len += 1; // Update our working length
                    has_carries = true; // Force another iteration
                }
                _ => {
                    // Check if ANY bit overflowed to continue the loop
                    has_carries = overflow_bits.par_iter().any(|&b| b == 1);
                }
            }
        }
    }
}

// Reuse AddAssign for Add
impl Add for NaturalNumber {
    type Output = NaturalNumber;

    fn add(mut self, other: NaturalNumber) -> NaturalNumber {
        // Just use the `+=` operator which routes directly to `add_assign`
        self += other;
        self
    }
}
