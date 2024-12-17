#![no_main]

extern crate num_bigint;
extern crate num_traits;

use num_bigint::{BigInt, BigUint, ToBigInt, ToBigUint};
use num_traits::{One, Zero};

// Safe operations for 64-bit
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

// Safe operations for 32-bit
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

/// Check if the player has enough "energy" (64-bit) and "focus" (32-bit) to perform an action.
fn validate_action_requirements(
    player_energy: u64,
    player_focus: u32,
    cost_energy: u64,
    cost_focus: u32,
) -> bool {
    if player_energy < cost_energy {
        return false;
    }
    if player_focus < cost_focus {
        return false;
    }
    true
}

/// Simulate combat damage using both 64-bit and 32-bit arithmetic.
/// For demonstration, let's say damage is (attack_power^2 / defense) as a rough formula.
fn compute_combat_damage(attack_power: u32, defense: u32) -> u64 {
    let squared = safe_mul_u32(attack_power, attack_power) as u64;
    let def = defense.max(1) as u64;
    safe_div_u64(squared, def)
}

/// Use BigUint for forging "Legendary Items" that require very large integer logic.
/// We artificially inflate computations to produce more opcodes.
fn forge_legendary_item_materials(base_materials: u64, rarity_factor: u32) -> u64 {
    let base_big = base_materials.to_biguint().unwrap_or(BigUint::zero());
    let rarity_big = rarity_factor.to_biguint().unwrap_or(BigUint::zero());
    let thousand_big = 1000u64.to_biguint().unwrap_or(BigUint::one());

    // item_cost = (base_materials * rarity_factor * 1000)
    let cost_big = &base_big * &rarity_big * &thousand_big;
    // Convert back to u64
    cost_big.try_into().unwrap_or(u64::MAX)
}

/// Experience (XP) leveling system that uses BigInt for partial calculations.
fn compute_experience_for_level(current_level: u32, xp_rate: u32) -> u64 {
    // XP needed for next level: (current_level^3 + xp_rate^2)
    let cl_big = current_level.to_bigint().unwrap_or(BigInt::zero());
    let xp_big = xp_rate.to_bigint().unwrap_or(BigInt::zero());

    let level_cubed = &cl_big * &cl_big * &cl_big;
    let xp_sqr = &xp_big * &xp_big;
    let needed = level_cubed + xp_sqr;

    // Convert to u64
    needed.try_into().unwrap_or(u64::MAX)
}

/// Complex fallback logic: If resources insufficient, try partial usage or alternate strategy.
/// For example, reduce half the cost, recalculate forging or damage.
fn partial_fallback(
    player_energy: u64,
    player_focus: u32,
    base_materials: u64,
    rarity_factor: u32,
    fallback_attempts: u32,
) -> u64 {
    if fallback_attempts == 0 {
        return 0;
    }

    // Attempt forging with half the base_materials or half the rarity_factor
    let half_materials = safe_div_u64(base_materials, 2);
    let half_rarity = safe_div_u32(rarity_factor, 2);

    let alt_cost = forge_legendary_item_materials(half_materials, half_rarity);
    if validate_action_requirements(player_energy, player_focus, alt_cost, half_rarity) {
        // Return some combined XOR result
        return combine_results(&[
            (alt_cost & 0xFFFFFFFF) as u32, // lower 32 bits
            half_rarity,
            (player_energy & 0xFFFFFFFF) as u32,
            player_focus,
        ]) as u64;
    } else {
        return partial_fallback(
            player_energy,
            player_focus,
            half_materials,
            half_rarity,
            fallback_attempts - 1,
        );
    }
}

/// Combine results to produce a single `u64`.
fn combine_results_64(results: &[u64]) -> u64 {
    let mut out = 0u64;
    for &r in results {
        out ^= r;
    }
    out
}

/// Combine `u32` results for partial fallback usage.
fn combine_results(results: &[u32]) -> u32 {
    let mut out = 0u32;
    for &r in results {
        out ^= r;
    }
    out
}

#[no_mangle]
pub fn main(player_energy: u64, player_focus: u32, base_materials: u64, rarity_factor: u32) -> u64 {
    // Step 1: Validate basic action requirements for forging a "Legendary Item"
    let forging_cost = forge_legendary_item_materials(base_materials, rarity_factor);
    let forging_focus_cost = safe_mul_u32(rarity_factor, 2); // e.g. forging consumes focus at 2x rarity

    if !validate_action_requirements(
        player_energy,
        player_focus,
        forging_cost,
        forging_focus_cost,
    ) {
        // Step 1a: Attempt partial fallback logic
        return partial_fallback(
            player_energy,
            player_focus,
            base_materials,
            rarity_factor,
            3,
        );
    }

    // Step 2: Simulate a short combat scenario with fixed stats for demonstration
    // e.g. Attack power = (rarity_factor + 50), defense = 100
    let attack_power = safe_add_u32(rarity_factor, 50);
    let damage_dealt = compute_combat_damage(attack_power, 100);

    // Step 3: Compute XP needed for next level
    let xp_needed = compute_experience_for_level(rarity_factor, 10);

    // Step 4: Combine results into a single 64-bit output
    combine_results_64(&[
        forging_cost,
        damage_dealt,
        xp_needed,
        player_energy,
        base_materials,
    ])
}
