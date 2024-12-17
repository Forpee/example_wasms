#![no_main]

extern crate num_bigint;
extern crate num_traits;

use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};

// Safe ops for u64
fn safe_add_u64(a: u64, b: u64) -> u64 {
    a.checked_add(b).unwrap_or(u64::MAX)
}
fn safe_sub_u64(a: u64, b: u64) -> u64 {
    a.checked_sub(b).unwrap_or(0)
}
fn safe_mul_u64(a: u64, b: u64) -> u64 {
    a.checked_mul(b).unwrap_or(u64::MAX)
}
fn safe_div_u64(a: u64, b: u64) -> u64 {
    if b == 0 {
        0
    } else {
        a / b
    }
}

// Safe ops for u32
fn safe_add_u32(a: u32, b: u32) -> u32 {
    a.checked_add(b).unwrap_or(u32::MAX)
}
fn safe_sub_u32(a: u32, b: u32) -> u32 {
    a.checked_sub(b).unwrap_or(0)
}
fn safe_mul_u32(a: u32, b: u32) -> u32 {
    a.checked_mul(b).unwrap_or(u32::MAX)
}
fn safe_div_u32(a: u32, b: u32) -> u32 {
    if b == 0 {
        0
    } else {
        a / b
    }
}

// Data provenance logic:
// For each product, we track environment flags (u32) and a 64-bit quality score,
// plus a 32-bit "certification bitmask" representing e.g. GMP, ISO standards, etc.

// We define multiple complexity steps:

/// 1) Validate an environment flag using bitwise checks:
///    We require that certain bits in environment_flag are set (bitmask check).
///    e.g. The environment_flag must have at least bit 0 and bit 2 set.
fn validate_environment(environment_flag: u32, required_mask: u32) -> bool {
    // Check if environment_flag AND required_mask == required_mask
    (environment_flag & required_mask) == required_mask
}

/// 2) Compute a "data lineage hash" using BigUint for complexity:
///    We'll mix the product_id, environment_flag, quality_score, and a prime factor in a large product.
fn compute_data_lineage_hash(
    product_id: u64,
    environment_flag: u32,
    quality_score: u64,
) -> BigUint {
    let product_id_big = product_id.to_biguint().unwrap_or(BigUint::zero());
    let env_big = environment_flag.to_biguint().unwrap_or(BigUint::zero());
    let quality_big = quality_score.to_biguint().unwrap_or(BigUint::zero());
    let prime_thing = 104729u64.to_biguint().unwrap(); // 104729 is a known prime

    // lineage_hash = (product_id_big << 3) * (env_big + 1) * (quality_big + 1) * prime
    // Using shifts, additions for complexity.
    let shifted = &product_id_big << 3;
    let mixed_env = &env_big + 1u32;
    let mixed_quality = &quality_big + 1u32;
    &shifted * &mixed_env * &mixed_quality * &prime_thing
}

/// 3) Further transformations on the certification bitmask:
///    We'll do rotates, shifts, and popcount.
fn transform_certification_bitmask(cert_mask: u32) -> u32 {
    // Rotate left by 5 bits
    let rotated_left = cert_mask.rotate_left(5);
    // Shift right by 2 bits
    let shifted_right = rotated_left >> 2;
    // Bitwise OR with the original
    let combined = shifted_right | cert_mask;
    // Popcount of combined
    let pop = combined.count_ones();
    // Return (combined XOR pop)
    combined ^ pop
}

/// 4) Combine results (like a final integrity check) using bitwise manipulations
///    For demonstration, let's create a final 64-bit value mixing lineage hash and partial fallback logic.
fn combine_final(
    lineage_hash: &BigUint,
    env_valid: bool,
    cert_transform: u32,
    quality_score: u64,
) -> u64 {
    // Convert BigUint hash to u64 by taking the lower 64 bits
    let lower_64 = lineage_hash.to_u64_digits();
    let lineage_lo = if !lower_64.is_empty() { lower_64[0] } else { 0 };

    // Some bitwise manipulations:
    let validity_bit = if env_valid { 1u64 } else { 0u64 };
    let masked_cert = cert_transform as u64;

    // demonstration: rotate lineage_lo left by 7 bits
    let rotate_left_7 = lineage_lo.rotate_left(7);

    // Then combine everything via XOR
    let x = rotate_left_7 ^ validity_bit;
    let y = masked_cert ^ quality_score;
    x ^ y
}

// Partial fallback: if environment isn't valid, we attempt to shift environment flags or reduce quality
// until valid or we run out of tries.
fn partial_fallback(
    environment_flag: u32,
    quality_score: u64,
    product_id: u64,
    certification_bitmask: u32,
    attempts: u32,
) -> u64 {
    if attempts == 0 {
        return 0;
    }

    // Try shifting environment_flag left by 1 to set new bits:
    let new_env = environment_flag << 1;
    if validate_environment(new_env, 0b101) {
        let lineage_hash = compute_data_lineage_hash(product_id, new_env, quality_score);
        let transform = transform_certification_bitmask(certification_bitmask);
        return combine_final(&lineage_hash, true, transform, quality_score);
    } else {
        // Attempt halving the quality score
        let half_quality = safe_div_u64(quality_score, 2);
        if validate_environment(environment_flag, 0b101) && half_quality > 0 {
            let lineage_hash =
                compute_data_lineage_hash(product_id, environment_flag, half_quality);
            let transform = transform_certification_bitmask(certification_bitmask);
            return combine_final(&lineage_hash, true, transform, half_quality);
        }
        // Recurse with attempts - 1
        partial_fallback(
            new_env,
            quality_score,
            product_id,
            certification_bitmask,
            attempts - 1,
        )
    }
}

#[no_mangle]
pub fn main(
    product_id: u64,
    environment_flag: u32,
    quality_score: u64,
    certification_bitmask: u32,
) -> u64 {
    // Step 1: Validate environment
    let required_mask = 0b101; // bits 0 and 2 must be set
    let env_valid = validate_environment(environment_flag, required_mask);

    if !env_valid {
        // Attempt partial fallback
        return partial_fallback(
            environment_flag,
            quality_score,
            product_id,
            certification_bitmask,
            3,
        );
    }

    // Step 2: Compute data lineage hash
    let lineage_hash = compute_data_lineage_hash(product_id, environment_flag, quality_score);

    // Step 3: Transform certification bitmask
    let cert_transform = transform_certification_bitmask(certification_bitmask);

    // Step 4: Combine final
    combine_final(&lineage_hash, env_valid, cert_transform, quality_score)
}
