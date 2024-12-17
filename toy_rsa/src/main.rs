#![no_main]

// Safe operations for u64
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

// Safe operations for i64
fn safe_add_i64(a: i64, b: i64) -> i64 {
    a.checked_add(b).unwrap_or(i64::MAX)
}

fn safe_sub_i64(a: i64, b: i64) -> i64 {
    a.checked_sub(b).unwrap_or(i64::MIN)
}

fn safe_mul_i64(a: i64, b: i64) -> i64 {
    a.checked_mul(b).unwrap_or(i64::MAX)
}

fn safe_div_i64(a: i64, b: i64) -> i64 {
    if b == 0 {
        0
    } else {
        a / b
    }
}

/// Basic check if a candidate is prime-ish using trial division
/// (Not robust for real crypto, but okay for demonstration).
/// Showcases multiple divisions.
fn is_prime_like(candidate: u64) -> bool {
    if candidate < 2 {
        return false;
    }
    // For demonstration, only trial divide up to sqrt(candidate)
    let mut i = 2u64;
    while i * i <= candidate {
        if safe_div_u64(candidate, i) * i == candidate {
            return false;
        }
        i += 1;
    }
    true
}

/// Compute gcd(a, b) using Euclid's algorithm, with divisions sprinkled in.
fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Modular exponentiation: base^exp mod modulus, featuring many divisions
/// This is just a standard "square-and-multiply" with safe operations.
fn mod_exp(base: u64, exp: u64, modulus: u64) -> u64 {
    if modulus == 0 {
        return 0;
    }
    let mut result = 1u64;
    let mut current = base % modulus;
    let mut e = exp;

    while e > 0 {
        if e & 1 == 1 {
            let tmp = safe_mul_u64(result, current);
            result = tmp % modulus;
        }
        let sq = safe_mul_u64(current, current);
        current = sq % modulus;
        e >>= 1;
    }
    result
}

/// Compute toy RSA modulus n = p*q, totient = (p-1)*(q-1), then do encryption of message^e mod n
/// Return 0 if invalid. We'll keep the logic flexible and add complexity.
fn toy_rsa_encrypt(p: u64, q: u64, e: u64, message: u64) -> u64 {
    if !is_prime_like(p) || !is_prime_like(q) {
        return 0;
    }
    let n = safe_mul_u64(p, q);

    // Totient is (p-1)*(q-1) (basic RSA assumption)
    let phi_p = safe_sub_u64(p, 1);
    let phi_q = safe_sub_u64(q, 1);
    let phi = safe_mul_u64(phi_p, phi_q);

    // Ensure gcd(e, phi) = 1 for valid keys
    if gcd_u64(e, phi) != 1 {
        return 0;
    }

    // Attempt encryption
    mod_exp(message, e, n)
}

/// Partial fallback attempt if prime checks fail or gcd checks fail
/// For demonstration, we try to slightly tweak p or q by dividing them in half,
/// and re-check if that yields a workable scenario.
fn partial_fallback(
    p_candidate: u64,
    q_candidate: u64,
    e: u64,
    message: u64,
    attempts: u64,
) -> u64 {
    if attempts == 0 {
        return 0;
    }

    let p_half = safe_div_u64(p_candidate, 2);
    let q_half = safe_div_u64(q_candidate, 2);

    let encrypted_half = toy_rsa_encrypt(p_half, q_half, e, message);
    if encrypted_half != 0 {
        return encrypted_half;
    } else {
        // Try recursively with fewer attempts
        partial_fallback(p_half, q_half, e, message, attempts - 1)
    }
}

/// Attempt Chinese Remainder Theorem (CRT) version of encryption
/// If use_crt = 1, do a toy encryption using CRT approach for demonstration.
fn toy_rsa_encrypt_crt(p: u64, q: u64, e: u64, message: u64) -> u64 {
    if !is_prime_like(p) || !is_prime_like(q) {
        return 0;
    }
    let n = safe_mul_u64(p, q);
    let p_enc = mod_exp(message, e, p);
    let q_enc = mod_exp(message, e, q);

    // Combine using naive CRT approach:
    // M = q*(q_inv mod p)*p_enc + p*(p_inv mod q)*q_enc  (mod n)
    // For demonstration, let's do simpler manipulations with divisions:
    let q_inv_mod_p = mod_inverse(q, p);
    let p_inv_mod_q = mod_inverse(p, q);
    if q_inv_mod_p == 0 || p_inv_mod_q == 0 {
        return 0;
    }

    let term1 = safe_mul_u64(q, q_inv_mod_p) % n;
    let partial1 = safe_mul_u64(term1, p_enc) % n;

    let term2 = safe_mul_u64(p, p_inv_mod_q) % n;
    let partial2 = safe_mul_u64(term2, q_enc) % n;

    safe_add_u64(partial1, partial2) % n
}

/// Compute modular inverse using Extended Euclidean Algorithm
/// Return 0 if inverse doesn't exist (which also showcases divisions).
fn mod_inverse(a: u64, m: u64) -> u64 {
    // Extended Euclid: find x,y s.t. a*x + m*y = gcd(a,m)
    // If gcd(a,m) = 1, then a*x ≡ 1 (mod m)
    if m == 0 {
        return 0;
    }
    let (g, x) = extended_gcd(a as i64, m as i64);
    if g != 1 {
        0
    } else {
        let mut inv = x % (m as i64);
        if inv < 0 {
            inv = safe_add_i64(inv, m as i64);
        }
        inv as u64
    }
}

/// Extended Euclidean Algorithm returning gcd(a, b), and coefficient x for 'a*x + b*y = gcd(a,b)'
/// This is done with signed operations (and divisions).
fn extended_gcd(a: i64, b: i64) -> (i64, i64) {
    if b == 0 {
        return (a, 1);
    }
    let (g, x1) = extended_gcd(b, a % b);
    // The "y" is not needed for just the inverse. We'll just store x part.
    let x = safe_sub_i64(0, safe_div_i64(a, b)) * x1;
    (g, x1 - x)
}

/// Combine results with XOR (similar pattern to previous code).
fn combine_results(results: &[u64]) -> u64 {
    let mut out = 0;
    for &r in results {
        out ^= r;
    }
    out
}

#[no_mangle]
pub fn main(p_candidate: u64, q_candidate: u64, e: u64, message: u64, use_crt: u64) -> u64 {
    // Step 1: Try standard toy RSA encryption
    let encrypted = toy_rsa_encrypt(p_candidate, q_candidate, e, message);
    if encrypted == 0 {
        // Step 2: Attempt partial fallback logic if standard encryption fails
        let fallback_encrypted = partial_fallback(p_candidate, q_candidate, e, message, 3);
        if fallback_encrypted == 0 {
            return 0;
        }
        return fallback_encrypted;
    }

    // Step 3: If user set use_crt == 1, optionally compute CRT-based encryption for demonstration
    let crt_encrypted = if use_crt == 1 {
        toy_rsa_encrypt_crt(p_candidate, q_candidate, e, message)
    } else {
        0
    };

    // Step 4: Combine results
    combine_results(&[encrypted, crt_encrypted])
}
