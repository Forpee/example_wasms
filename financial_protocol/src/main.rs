#![no_main]

// We assume you have added `num-bigint = "0.4"` in Cargo.toml
extern crate num_bigint;
extern crate num_traits;

use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};

// Safe operations for u32
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

// Safe operations for i32
fn safe_add_i32(a: i32, b: i32) -> i32 {
    a.checked_add(b).unwrap_or(i32::MAX)
}

fn safe_sub_i32(a: i32, b: i32) -> i32 {
    a.checked_sub(b).unwrap_or(i32::MIN)
}

fn safe_mul_i32(a: i32, b: i32) -> i32 {
    a.checked_mul(b).unwrap_or(i32::MAX)
}

fn safe_div_i32(a: i32, b: i32) -> i32 {
    if b == 0 {
        0
    } else {
        a / b
    }
}

/// Validate if the collateral is sufficient for the borrowed amount
/// Enforce a 200% collateral ratio for "healthy" loans.
fn validate_loan_health(collateral: u32, borrowed: u32) -> bool {
    if borrowed == 0 {
        return false;
    }
    let ratio = safe_div_u32(safe_mul_u32(collateral, 100), borrowed);
    ratio >= 200
}

/// Compute interest in basis points (bps), with more complex logic and loops:
/// We simulate compounding per time slice to increase complexity.
fn compute_compound_interest(borrowed: u32, annual_interest_bps: u32, time_slices: u32) -> u32 {
    // We'll do naive compounding in steps. For each slice, interest = borrowed*(annual_interest_bps/10000)* (1/time_slices)
    // Then borrowed += interest. Return final borrowed - original as the total interest accrued.
    if time_slices == 0 {
        return 0;
    }
    let mut principal = borrowed;
    let fraction_bps = safe_div_u32(annual_interest_bps, time_slices);

    for _ in 0..time_slices {
        let rate = safe_div_u32(fraction_bps, 10000);
        let slice_interest = safe_mul_u32(principal, rate);
        principal = safe_add_u32(principal, slice_interest);
    }
    safe_sub_u32(principal, borrowed)
}

/// Compute staking rewards using big integer logic for complexity:
/// We'll treat the staked amount as a BigUint, do some arbitrary expansions, then reduce back to u32.
fn compute_staking_rewards_bigint(
    collateral: u32,
    stake_ratio: u32,
    reward_rate_bps: u32,
    time_slices: u32,
) -> u32 {
    let collateral_big = collateral.to_biguint().unwrap_or(BigUint::zero());
    let ratio_big = stake_ratio.to_biguint().unwrap_or(BigUint::zero());
    let hundred_big = 100u32.to_biguint().unwrap_or(BigUint::one());
    let reward_bps_big = reward_rate_bps.to_biguint().unwrap_or(BigUint::zero());
    let ten_thousand_big = 10000u32.to_biguint().unwrap_or(BigUint::one());

    // staked_amount = (collateral * stake_ratio)/100 as BigUint
    let staked = (&collateral_big * &ratio_big) / &hundred_big;
    // total_reward_rate = (reward_rate_bps/time_slices)/10000 in BigUint
    // We'll compound similarly over time_slices
    let mut current_staked = staked.clone();
    for _ in 0..time_slices {
        let partial_rate = &reward_bps_big / &ten_thousand_big / time_slices;
        // biguint does not do fractional divides precisely, so we'll keep it integer-limited
        // But we will artificially simulate partial compounding
        let yield_part = &current_staked * &partial_rate;
        current_staked = &current_staked + yield_part;
    }
    // The difference is the reward
    let reward = if current_staked > staked {
        &current_staked - &staked
    } else {
        BigUint::zero()
    };
    // Convert back to u32
    reward.try_into().unwrap_or(u32::MAX)
}

/// Simulate a complex liquidity pool shares mechanism with multiple steps:
/// 1. There's a base decay each block
/// 2. There's a performance fee (subtraction)
/// 3. Optional partial fallback if shares drop below a threshold
fn simulate_liquidity_pool_shares_complex(
    shares: u32,
    time_slices: u32,
    decay_bps: u32,
    performance_fee_bps: u32,
) -> u32 {
    if time_slices == 0 {
        return shares;
    }

    let mut current_shares = shares;
    for _ in 0..time_slices {
        // Decay
        let decay_rate = safe_div_u32(decay_bps, 10000);
        let decay_amount = safe_mul_u32(current_shares, decay_rate);
        current_shares = safe_sub_u32(current_shares, decay_amount);

        // Performance fee
        let fee_rate = safe_div_u32(performance_fee_bps, 10000);
        let fee_amount = safe_mul_u32(current_shares, fee_rate);
        current_shares = safe_sub_u32(current_shares, fee_amount);

        // If shares drop below 100, do partial fallback: attempt re-stake half
        if current_shares < 100 {
            let half_stake = safe_div_u32(current_shares, 2);
            current_shares = safe_add_u32(current_shares, half_stake); // artificially re-stake half
        }
    }
    current_shares
}

/// Attempt a partial fallback to fix a loan that isn't healthy:
/// We do multiple stages: reduce borrowed, re-check, compound interest again,
/// and combine all partial results with XOR at the end.
fn partial_fallback_loan(
    collateral: u32,
    borrowed: u32,
    annual_interest_bps: u32,
    stake_ratio: u32,
    reward_rate_bps: u32,
) -> u32 {
    // 1) Half the borrowed
    let half_borrowed = safe_div_u32(borrowed, 2);
    if validate_loan_health(collateral, half_borrowed) {
        let interest = compute_compound_interest(half_borrowed, annual_interest_bps, 3);
        let staking = compute_staking_rewards_bigint(collateral, stake_ratio, reward_rate_bps, 3);
        return combine_results(&[half_borrowed, interest, staking, collateral]);
    }

    // 2) Quarter the borrowed
    let quarter_borrowed = safe_div_u32(half_borrowed, 2);
    if validate_loan_health(collateral, quarter_borrowed) {
        let interest = compute_compound_interest(quarter_borrowed, annual_interest_bps, 3);
        let staking = compute_staking_rewards_bigint(collateral, stake_ratio, reward_rate_bps, 3);
        return combine_results(&[quarter_borrowed, interest, staking, collateral]);
    }

    // If still not healthy:
    0
}

/// Combine multiple results with XOR for final single-u32 output.
fn combine_results(results: &[u32]) -> u32 {
    let mut out = 0u32;
    for &r in results {
        out ^= r;
    }
    out
}

#[no_mangle]
pub fn main(
    collateral_amount: u32,
    borrowed_amount: u32,
    stake_ratio: u32,
    annual_interest_bps: u32,
) -> u32 {
    // Step 1: Validate the loan
    if !validate_loan_health(collateral_amount, borrowed_amount) {
        // Step 1a: Attempt partial fallback if invalid
        return partial_fallback_loan(
            collateral_amount,
            borrowed_amount,
            annual_interest_bps,
            stake_ratio,
            600,
        );
    }

    // Step 2: Calculate compound interest over 5 time slices
    let interest_accrued = compute_compound_interest(borrowed_amount, annual_interest_bps, 5);

    // Step 3: Calculate staking rewards with BigUint-based compounding
    let staking_rewards = compute_staking_rewards_bigint(collateral_amount, stake_ratio, 600, 5);

    // Step 4: Simulate a more complex liquidity pool scenario for further complexity
    let final_shares = simulate_liquidity_pool_shares_complex(2000, 5, 100, 50);

    // Combine everything
    combine_results(&[
        borrowed_amount,
        interest_accrued,
        staking_rewards,
        final_shares,
        collateral_amount,
    ])
}
