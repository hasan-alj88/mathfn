/// Adds two slices together and returns a new Vec
pub(crate) fn add_slices(a: &[u128], b: &[u128]) -> Vec<u128> {
    let max_len = a.len().max(b.len());
    let mut out = vec![0u128; max_len];
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

/// Adds a slice into an existing mutable array starting at a specific index offset
pub(crate) fn add_into(target: &mut [u128], addend: &[u128], offset: usize) {
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
pub(crate) fn sub_from(target: &mut [u128], sub: &[u128]) {
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
pub(crate) fn mul_wide(a: u128, b: u128) -> (u128, u128) {
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
