#![no_main]

// Safety helpers to prevent overflow (very basic checks)
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

fn safe_add(a: u64, b: u64) -> u64 {
    a.checked_add(b).unwrap_or(u64::MAX)
}

fn safe_sub(a: u64, b: u64) -> u64 {
    a.checked_sub(b).unwrap_or(0)
}

// Validate the swap amount
fn validate_swap_amount(swap_amount: u64, user_balance: u64) -> bool {
    swap_amount > 0 && swap_amount <= user_balance
}

// Simulate a historical price feed (in a real scenario this might be external)
// Here we just create a pseudo "history" and compute a weighted average price.
fn simulate_historical_price_data(current_price: u64) -> u64 {
    // Pretend we have a small sliding window of past prices
    let past_prices = [
        current_price.saturating_sub(50),
        current_price,
        current_price.saturating_add(25),
    ];
    let weights = [1, 2, 1]; // Weighted more towards the current price
    let mut weighted_sum = 0;
    let mut weight_total = 0;
    for i in 0..past_prices.len() {
        weighted_sum = safe_add(weighted_sum, safe_mul(past_prices[i], weights[i]));
        weight_total = safe_add(weight_total, weights[i]);
    }
    safe_div(weighted_sum, weight_total)
}

// Dynamically adjust the base fee depending on trade size and historical volatility
fn adjust_fee(
    swap_amount: u64,
    pool_input_reserve: u64,
    historical_price: u64,
    current_price: u64,
) -> (u64, u64) {
    // Base fee: 0.3%
    let base_fee_numerator = 3;
    let base_fee_denominator = 1000;

    // Increase fee if trade is large compared to input reserve
    let large_trade_threshold = safe_div(pool_input_reserve, 10);
    let mut fee_num = base_fee_numerator;
    let fee_den = base_fee_denominator;

    if swap_amount > large_trade_threshold {
        fee_num = safe_mul(fee_num, 2); // Double fee for large trades
    }

    // Increase fee if current price deviates strongly from historical price (volatility)
    let price_diff = if current_price > historical_price {
        safe_sub(current_price, historical_price)
    } else {
        safe_sub(historical_price, current_price)
    };

    let volatility_ratio = safe_mul(price_diff, 1000) / (historical_price.max(1));
    if volatility_ratio > 50 {
        // If volatility > 5%, increase fee further
        fee_num = safe_add(fee_num, 2);
    }

    (fee_num, fee_den)
}

// Calculate effective input after fee
fn effective_input_after_fee(input_amount: u64, fee_numerator: u64, fee_denominator: u64) -> u64 {
    // effective_input = input_amount * fee_denominator / (fee_denominator + fee_numerator)
    let total = safe_add(fee_denominator, fee_numerator);
    if total == 0 {
        return 0;
    }
    safe_div(safe_mul(input_amount, fee_denominator), total)
}

// Calculate swap output using the constant product formula
fn calculate_swap_output_with_fee(
    input_amount: u64,
    input_reserve: u64,
    output_reserve: u64,
    fee_numerator: u64,
    fee_denominator: u64,
) -> u64 {
    let effective_input = effective_input_after_fee(input_amount, fee_numerator, fee_denominator);
    // dy = (effective_input * output_reserve) / (input_reserve + effective_input)
    let denom = safe_add(input_reserve, effective_input);
    if denom == 0 {
        return 0;
    }
    safe_div(safe_mul(effective_input, output_reserve), denom)
}

// Calculate slippage as percentage difference
fn calculate_slippage(input_amount: u64, input_reserve: u64, output_reserve: u64) -> u64 {
    let initial_price = if output_reserve == 0 {
        return 100; // If no liquidity, slippage is effectively infinite
    } else {
        safe_div(safe_mul(input_reserve, 1_000_000), output_reserve)
    };

    let new_input_reserve = safe_add(input_reserve, input_amount);
    let denom = safe_add(input_reserve, input_amount);
    let next_output = if denom == 0 {
        return 100; // No meaningful trade possible
    } else {
        safe_sub(
            output_reserve,
            safe_div(safe_mul(input_amount, output_reserve), denom),
        )
    };

    if next_output == 0 {
        return 100; // Drained pool scenario
    }

    let new_price = safe_div(safe_mul(new_input_reserve, 1_000_000), next_output);

    if new_price < initial_price {
        safe_div(
            safe_mul(safe_sub(initial_price, new_price), 100),
            initial_price,
        )
    } else {
        0
    }
}

// Check if slippage is within tolerance
fn check_slippage_tolerance(slippage: u64, max_slippage: u64) -> bool {
    slippage <= max_slippage
}

// Calculate the pool value in terms of the input asset
fn calculate_pool_value(input_reserve: u64, output_reserve: u64, price: u64) -> u64 {
    // Pool value = input_reserve + (output_reserve * price / 1_000_000)
    safe_add(
        input_reserve,
        safe_div(safe_mul(output_reserve, price), 1_000_000),
    )
}

// Fees collected from the user
fn calculate_fees_collected(input_amount: u64, fee_numerator: u64, fee_denominator: u64) -> u64 {
    let total = safe_add(fee_denominator, fee_numerator);
    if total == 0 {
        return 0;
    }
    safe_div(safe_mul(input_amount, fee_numerator), total)
}

// Simulate pool state after trade
fn simulate_pool_state(
    input_reserve: u64,
    output_reserve: u64,
    swap_amount: u64,
    output_amount: u64,
) -> (u64, u64) {
    let new_input_reserve = safe_add(input_reserve, swap_amount);
    let new_output_reserve = if output_reserve > output_amount {
        safe_sub(output_reserve, output_amount)
    } else {
        0
    };
    (new_input_reserve, new_output_reserve)
}

// Check if the pool remains healthy after the swap:
// For complexity, we impose a minimum liquidity rule.
fn check_pool_health(new_input_reserve: u64, new_output_reserve: u64) -> bool {
    // Suppose we require at least 1000 units of each token in the pool after trade
    new_input_reserve > 1000 && new_output_reserve > 1000
}

// Distribute fees into different "funds" for complexity demonstration
fn distribute_fees(fees: u64) -> (u64, u64, u64) {
    // 50% to liquidity providers
    let lp_share = safe_div(fees, 2);
    // 30% to a protocol treasury
    let treasury_share = safe_div(safe_mul(fees, 3), 10);
    // Remaining 20% to an insurance fund
    let used = safe_add(lp_share, treasury_share);
    let insurance_share = if fees > used { safe_sub(fees, used) } else { 0 };
    (lp_share, treasury_share, insurance_share)
}

// Attempt a partial trade if full trade conditions fail
// This is just a demonstration of complexity; we return a reduced output.
fn attempt_partial_trade(
    swap_amount: u64,
    user_input_balance: u64,
    pool_input_reserve: u64,
    pool_output_reserve: u64,
    fee_numerator: u64,
    fee_denominator: u64,
    max_slippage: u64,
) -> u64 {
    // Try half the swap amount
    let half_amount = safe_div(swap_amount, 2);
    if half_amount == 0 || half_amount > user_input_balance {
        return 0;
    }

    let output_half = calculate_swap_output_with_fee(
        half_amount,
        pool_input_reserve,
        pool_output_reserve,
        fee_numerator,
        fee_denominator,
    );

    let slippage_half = calculate_slippage(half_amount, pool_input_reserve, pool_output_reserve);
    if !check_slippage_tolerance(slippage_half, max_slippage) {
        return 0;
    }

    output_half
}

#[no_mangle]
pub fn main(
    user_input_balance: u64,
    pool_input_reserve: u64,
    pool_output_reserve: u64,
    swap_amount: u64,
    price: u64,
) -> u64 {
    // Step 1: Validate the swap amount
    if !validate_swap_amount(swap_amount, user_input_balance) {
        return 0;
    }

    // Step 2: Simulate historical data and get a historical price anchor
    let historical_price = simulate_historical_price_data(price);

    // Step 3: Adjust fee dynamically
    let (fee_numerator, fee_denominator) =
        adjust_fee(swap_amount, pool_input_reserve, historical_price, price);

    // Step 4: Calculate the output amount with fee
    let output_amount = calculate_swap_output_with_fee(
        swap_amount,
        pool_input_reserve,
        pool_output_reserve,
        fee_numerator,
        fee_denominator,
    );

    // Step 5: Calculate slippage
    let slippage = calculate_slippage(swap_amount, pool_input_reserve, pool_output_reserve);

    // Step 6: Define a max slippage tolerance (very strict)
    let max_slippage = 5; // 5%

    // Step 7: Check slippage tolerance
    if !check_slippage_tolerance(slippage, max_slippage) {
        // Attempt a partial trade for complexity demonstration if full fails
        let partial_output = attempt_partial_trade(
            swap_amount,
            user_input_balance,
            pool_input_reserve,
            pool_output_reserve,
            fee_numerator,
            fee_denominator,
            max_slippage,
        );
        if partial_output == 0 {
            return 0;
        }

        // Still produce a value, but reflect partial trade scenario:
        let fees =
            calculate_fees_collected(safe_div(swap_amount, 2), fee_numerator, fee_denominator);
        let (new_input_reserve, new_output_reserve) = simulate_pool_state(
            pool_input_reserve,
            pool_output_reserve,
            safe_div(swap_amount, 2),
            partial_output,
        );
        let updated_pool_value = calculate_pool_value(new_input_reserve, new_output_reserve, price);
        let (lp_share, treasury, insurance) = distribute_fees(fees);

        return partial_output
            ^ slippage
            ^ fees
            ^ updated_pool_value
            ^ lp_share
            ^ treasury
            ^ insurance;
    }

    // Step 8: Calculate fees collected
    let fees = calculate_fees_collected(swap_amount, fee_numerator, fee_denominator);

    // Step 9: Simulate new pool state
    let (new_input_reserve, new_output_reserve) = simulate_pool_state(
        pool_input_reserve,
        pool_output_reserve,
        swap_amount,
        output_amount,
    );

    // Step 10: Check pool health
    if !check_pool_health(new_input_reserve, new_output_reserve) {
        // If not healthy, attempt partial trade as fallback
        let partial_output = attempt_partial_trade(
            swap_amount,
            user_input_balance,
            pool_input_reserve,
            pool_output_reserve,
            fee_numerator,
            fee_denominator,
            max_slippage,
        );
        if partial_output == 0 {
            return 0;
        }

        let fees_partial =
            calculate_fees_collected(safe_div(swap_amount, 2), fee_numerator, fee_denominator);
        let (p_input_reserve, p_output_reserve) = simulate_pool_state(
            pool_input_reserve,
            pool_output_reserve,
            safe_div(swap_amount, 2),
            partial_output,
        );
        let updated_pool_value_partial =
            calculate_pool_value(p_input_reserve, p_output_reserve, price);
        let (lp_share, treasury, insurance) = distribute_fees(fees_partial);

        return partial_output
            ^ slippage
            ^ fees_partial
            ^ updated_pool_value_partial
            ^ lp_share
            ^ treasury
            ^ insurance;
    }

    // Step 11: Calculate the updated pool value
    let updated_pool_value = calculate_pool_value(new_input_reserve, new_output_reserve, price);

    // Step 12: Distribute fees for complexity
    let (lp_share, treasury, insurance) = distribute_fees(fees);

    // Combine results
    output_amount ^ slippage ^ fees ^ updated_pool_value ^ lp_share ^ treasury ^ insurance
}
