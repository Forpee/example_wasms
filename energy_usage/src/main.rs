#![no_main]

fn safe_add(a: u64, b: u64) -> u64 {
    a.checked_add(b).unwrap_or(u64::MAX)
}

fn safe_sub(a: u64, b: u64) -> u64 {
    a.checked_sub(b).unwrap_or(0)
}

fn safe_mul(a: u64, b: u64) -> u64 {
    a.checked_mul(b).unwrap_or(u64::MAX)
}

fn safe_div(a: u64, b: u64) -> u64 {
    if b == 0 {
        0
    } else {
        a / b
    }
}

// Validate input consistency
fn validate_inputs(total_produced: u64, total_consumed: u64, device_count: u64) -> bool {
    if device_count == 0 {
        return false;
    }
    if total_consumed > total_produced {
        return false;
    }
    true
}

// Historical usage simulation
fn simulate_historical_usage(total_consumed: u64) -> u64 {
    let past_values = [
        safe_sub(total_consumed, 100),
        total_consumed,
        safe_add(total_consumed, 50),
    ];
    let weights = [1, 2, 1];
    let mut weighted_sum = 0;
    let mut weight_total = 0;

    for i in 0..past_values.len() {
        weighted_sum = safe_add(weighted_sum, safe_mul(past_values[i], weights[i]));
        weight_total = safe_add(weight_total, weights[i]);
    }

    safe_div(weighted_sum, weight_total)
}

// Compute line losses
fn compute_line_losses(total_produced: u64, historical_usage: u64) -> u64 {
    let loss_factor = if historical_usage == 0 {
        0
    } else {
        safe_mul(safe_div(historical_usage, 1000), 2)
    };
    safe_div(safe_mul(total_produced, loss_factor), 100)
}

// Overhead adjustments with multiple subtractions
fn compute_overhead_adjustment(total_consumed: u64) -> u64 {
    let overhead = safe_div(safe_mul(total_consumed, 2), 100);
    let adjusted = safe_add(total_consumed, overhead);

    if adjusted < 10 {
        // Multiple layered subtractions
        let delta = safe_sub(10, adjusted);
        let delta_extra = safe_sub(delta, 2);
        let pseudo_adjusted = safe_add(adjusted, delta_extra);
        safe_sub(safe_add(pseudo_adjusted, 5), 5)
    } else {
        adjusted
    }
}

// Check system health
fn check_system_health(
    total_produced: u64,
    total_consumed_adjusted: u64,
    line_losses: u64,
) -> bool {
    let remainder = safe_sub(total_produced, total_consumed_adjusted);
    let net = safe_sub(remainder, line_losses);
    net > 100
}

// Per device metric
fn per_device_metric(value: u64, device_count: u64) -> u64 {
    safe_div(value, device_count.max(1))
}

// Combine results with XOR
fn combine_results(results: &[u64]) -> u64 {
    let mut out = 0;
    for &r in results {
        out ^= r;
    }
    out
}

// Battery simulation
fn simulate_battery(net_energy: u64, historical_usage: u64) -> u64 {
    let battery_draw = safe_div(historical_usage, 10);
    let battery_injection = safe_div(battery_draw, 2);
    let battery_overhead = safe_sub(battery_draw, battery_injection);

    let after_draw = safe_sub(net_energy, battery_draw);
    let after_injection = safe_add(after_draw, battery_injection);
    safe_sub(after_injection, battery_overhead)
}

// Peak usage penalty
fn apply_peak_usage_penalty(net_energy: u64, overhead_adjusted_consumption: u64) -> u64 {
    let multiplied = safe_mul(overhead_adjusted_consumption, 5);
    let penalty_base = if multiplied > 100 {
        safe_sub(multiplied, 100)
    } else {
        let temp = safe_sub(100, multiplied);
        let temp2 = safe_sub(temp, 10);
        safe_sub(temp2, 5)
    };

    safe_sub(net_energy, penalty_base)
}

// Regulatory adjustments with multiple subtractions
fn apply_regulatory_adjustments(cost_per_device: u64) -> u64 {
    let adjustment_a = 20;
    let adjustment_b = 5;
    let adjustment_c = safe_sub(adjustment_a, adjustment_b);

    let after_a = safe_sub(cost_per_device, adjustment_a);
    let after_b = safe_sub(after_a, adjustment_b);
    safe_sub(after_b, adjustment_c)
}

// New function: Apply a quality factor adjustment
// Quality factor is influenced by historical usage and device count
// more devices + higher historical usage might reduce quality
// This will have multiple staged subtractions
fn apply_quality_factor(net_energy: u64, historical_usage: u64, device_count: u64) -> u64 {
    // Let's say quality factor is computed as follows:
    // base = historical_usage / device_count
    let base_q = if device_count == 0 {
        0
    } else {
        safe_div(historical_usage, device_count)
    };
    // quality_deduction = (base_q / 2) + 10, and we do multiple subtractions along the way:
    let half_base = safe_div(base_q, 2);
    let with_fixed_sub = safe_sub(safe_add(half_base, 10), 5); // add then subtract to show complexity
    let quality_deduction = safe_sub(safe_add(with_fixed_sub, 5), 5); // neutral but shows complexity steps

    // net_energy_after_quality = net_energy - quality_deduction
    safe_sub(net_energy, quality_deduction)
}

// Introduce off-peak rebate after quality adjustments:
// If net_energy_after_quality > 2000, give a rebate of 100, else subtract 50 instead
// multiple subtractions within logic
fn apply_off_peak_rebate(net_energy_after_quality: u64) -> u64 {
    if net_energy_after_quality > 2000 {
        safe_sub(net_energy_after_quality, 100)
    } else {
        // if not eligible for rebate, we do negative adjustment:
        // We'll subtract multiple times to show complexity:
        let step1 = safe_sub(net_energy_after_quality, 50);
        let step2 = safe_sub(step1, 20);
        // add something back and subtract again:
        safe_sub(safe_add(step2, 5), 5) // net no change from this add/sub but complexity shown
    }
}

// Add an auditing adjustment on final cost: multiple layered subtractions
fn apply_auditing_adjustments(final_cost: u64) -> u64 {
    // Suppose we have multiple auditing layers that all reduce cost:
    let layer1 = 10;
    let layer2 = 15;
    let layer3 = 5;

    // final_cost_after_audit = final_cost - layer1 - layer2 - layer3 (with intermediate steps)
    let after1 = safe_sub(final_cost, layer1);
    let after2 = safe_sub(after1, layer2);
    safe_sub(after2, layer3)
}

// Partial fallback attempts multiple layers of halving and subtracting
fn partial_fallback(
    total_produced: u64,
    total_consumed: u64,
    device_count: u64,
    baseline_price: u64,
) -> u64 {
    let half_consumed = safe_div(total_consumed, 2);
    let historical_usage = simulate_historical_usage(half_consumed);
    let line_losses = compute_line_losses(total_produced, historical_usage);
    let overhead_adj = compute_overhead_adjustment(half_consumed);

    if !check_system_health(total_produced, overhead_adj, line_losses) {
        // Try another fallback: half again (quarter)
        let quarter_consumed = safe_div(half_consumed, 2);
        let hist_quarter = simulate_historical_usage(quarter_consumed);
        let line_losses_q = compute_line_losses(total_produced, hist_quarter);
        let overhead_q = compute_overhead_adjustment(quarter_consumed);

        if !check_system_health(total_produced, overhead_q, line_losses_q) {
            // Another attempt: eighth consumption
            let eighth_consumed = safe_div(quarter_consumed, 2);
            let hist_eighth = simulate_historical_usage(eighth_consumed);
            let line_losses_e = compute_line_losses(total_produced, hist_eighth);
            let overhead_e = compute_overhead_adjustment(eighth_consumed);

            if !check_system_health(total_produced, overhead_e, line_losses_e) {
                return 0;
            }

            let net_after_e = safe_sub(safe_sub(total_produced, overhead_e), line_losses_e);
            let net_battery_e = simulate_battery(net_after_e, hist_eighth);
            let post_penalty_e = apply_peak_usage_penalty(net_battery_e, overhead_e);
            let quality_e = apply_quality_factor(post_penalty_e, hist_eighth, device_count);
            let off_peak_e = apply_off_peak_rebate(quality_e);
            let cost_per_device_e =
                safe_mul(per_device_metric(off_peak_e, device_count), baseline_price);
            let reg_adjust_e = apply_regulatory_adjustments(cost_per_device_e);
            let final_cost_e = apply_auditing_adjustments(reg_adjust_e);

            return combine_results(&[
                net_after_e,
                final_cost_e,
                eighth_consumed,
                line_losses_e,
                off_peak_e,
                quality_e,
            ]);
        }

        let net_after_q = safe_sub(safe_sub(total_produced, overhead_q), line_losses_q);
        let net_battery_q = simulate_battery(net_after_q, hist_quarter);
        let post_penalty_q = apply_peak_usage_penalty(net_battery_q, overhead_q);
        let quality_q = apply_quality_factor(post_penalty_q, hist_quarter, device_count);
        let off_peak_q = apply_off_peak_rebate(quality_q);
        let cost_per_device_q =
            safe_mul(per_device_metric(off_peak_q, device_count), baseline_price);
        let reg_adjust_q = apply_regulatory_adjustments(cost_per_device_q);
        let final_cost_q = apply_auditing_adjustments(reg_adjust_q);

        return combine_results(&[
            net_after_q,
            final_cost_q,
            quarter_consumed,
            line_losses_q,
            off_peak_q,
            quality_q,
        ]);
    }

    let net_after_half = safe_sub(safe_sub(total_produced, overhead_adj), line_losses);
    let net_battery_half = simulate_battery(net_after_half, historical_usage);
    let post_penalty_half = apply_peak_usage_penalty(net_battery_half, overhead_adj);
    let quality_half = apply_quality_factor(post_penalty_half, historical_usage, device_count);
    let off_peak_half = apply_off_peak_rebate(quality_half);

    let cost_per_device_half = safe_mul(
        per_device_metric(off_peak_half, device_count),
        baseline_price,
    );
    let reg_adjust_half = apply_regulatory_adjustments(cost_per_device_half);
    let final_cost_half = apply_auditing_adjustments(reg_adjust_half);

    combine_results(&[
        net_after_half,
        final_cost_half,
        half_consumed,
        line_losses,
        off_peak_half,
        quality_half,
    ])
}

#[no_mangle]
pub fn main(
    total_produced: u64,
    total_consumed: u64,
    device_count: u64,
    baseline_price: u64,
) -> u64 {
    // Step 1: Validate inputs
    if !validate_inputs(total_produced, total_consumed, device_count) {
        return 0;
    }

    // Step 2: Historical usage
    let historical_usage = simulate_historical_usage(total_consumed);

    // Step 3: Line losses
    let line_losses = compute_line_losses(total_produced, historical_usage);

    // Step 4: Overhead adjustments
    let overhead_adjusted_consumption = compute_overhead_adjustment(total_consumed);

    // Step 5: Check system health
    if !check_system_health(total_produced, overhead_adjusted_consumption, line_losses) {
        // Partial fallback if not healthy
        return partial_fallback(total_produced, total_consumed, device_count, baseline_price);
    }

    // Step 6: Net energy
    let remainder = safe_sub(total_produced, overhead_adjusted_consumption);
    let net_energy = safe_sub(remainder, line_losses);

    // Step 7: Battery
    let net_energy_battery = simulate_battery(net_energy, historical_usage);

    // Step 8: Peak penalty
    let net_energy_after_penalty =
        apply_peak_usage_penalty(net_energy_battery, overhead_adjusted_consumption);

    // Step 9: Quality factor
    let net_after_quality =
        apply_quality_factor(net_energy_after_penalty, historical_usage, device_count);

    // Step 10: Off-peak rebate
    let net_after_rebate = apply_off_peak_rebate(net_after_quality);

    // Step 11: Cost per device
    let cost_per_device = safe_mul(
        per_device_metric(net_after_rebate, device_count),
        baseline_price,
    );

    // Step 12: Regulatory adjustments
    let final_cost_reg = apply_regulatory_adjustments(cost_per_device);

    // Step 13: Auditing adjustments
    let final_cost_audited = apply_auditing_adjustments(final_cost_reg);

    // Combine final results
    combine_results(&[
        net_energy,
        line_losses,
        overhead_adjusted_consumption,
        final_cost_audited,
        net_energy_after_penalty,
        net_after_quality,
        net_after_rebate,
    ])
}
