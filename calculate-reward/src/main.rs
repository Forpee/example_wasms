#![no_main]

fn safe_add(a: u32, b: u32) -> u32 {
    a.checked_add(b).unwrap_or(u32::MAX)
}

fn safe_sub(a: u32, b: u32) -> u32 {
    a.checked_sub(b).unwrap_or(0)
}

fn safe_mul(a: u32, b: u32) -> u32 {
    a.checked_mul(b).unwrap_or(u32::MAX)
}

/// Simulates reward calculation in a DeFi liquidity pool.
/// - `stake`: amount staked by user
/// - `duration_boost`: boost based on how long the stake was held
/// - `volume_boost`: optional boost from trade volume (can be 0)
/// - `penalty`: optional penalty for early withdrawal or protocol downgrade
#[no_mangle]
pub fn main(stake: u32, duration_boost: u32, volume_boost: u32, penalty: u32) -> u32 {
    // Base reward = stake * duration_boost
    let base_reward = safe_mul(stake, duration_boost);

    // Volume adjusted = base_reward + (base_reward * volume_boost)
    let volume_bonus = safe_mul(base_reward, volume_boost);
    let total_reward = safe_add(base_reward, volume_bonus);

    // Apply penalty
    safe_sub(total_reward, penalty)
}
