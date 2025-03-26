use miller_rabin::is_prime;
use num_bigint::{BigUint, RandBigInt};
use num_traits::One;
use rand::rng;
use crate::asymmetric_encryption::utils::{big_int_mod_inverse, bigint_gcd};

fn generate_prime(bits: u64, rounds: usize) -> BigUint {
let mut rng = rng();
    loop {
        let candidate = rng.gen_biguint(bits);
        if is_prime(&candidate, rounds) {
            return candidate;
        }
    }
}

pub fn rsa_generate_key_pair(bits: u64, rounds: usize) -> (BigUint, BigUint, BigUint) {
    let e = BigUint::from(65537u32);
    let p = generate_prime(bits/2, rounds);
    let q = generate_prime(bits/2, rounds);
    let n = &p * &q;
    let phi = (&p - BigUint::one()) * (&q - BigUint::one());
    if bigint_gcd(&e, &phi) != BigUint::one() {
        return rsa_generate_key_pair(bits, rounds); // Retry if e is not coprime
    }
    let d = big_int_mod_inverse(e.clone(), phi.clone()).expect("No modular inverse found");
    (n, e, d)
}