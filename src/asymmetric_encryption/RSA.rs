use miller_rabin::is_prime;
use num::Integer;
use num_bigint::{BigUint, RandBigInt};
use num_traits::One;
use rand::rngs::OsRng;
use crate::asymmetric_encryption::utils::{big_int_mod_inverse, bigint_gcd};
use rand::thread_rng;

fn generate_prime(bits: u64, rounds: usize) -> BigUint {
    let mut rng = OsRng;
    loop {
        let mut candidate = rng.gen_biguint(bits - 1);
        candidate.set_bit(bits - 1, true);
        if candidate.is_even(){
            candidate += BigUint::one();
        }
        if is_prime(&candidate, rounds) {
            return candidate;
        }
    }
}
pub fn rsa_generate_key_pair(bits: u64, rounds: usize) -> (BigUint, BigUint, BigUint) {
    let e = BigUint::from(65537u32);
    let mut max_attemps  = 100;
    let mut attemps = 0;
    loop {
        attemps += 1;
        if attemps > max_attemps {
            panic!("Failed to generate RSA key pair after {} attemps.", max_attemps);
        }
        let p = generate_prime(bits / 2, rounds);
        let q = generate_prime(bits / 2, rounds);

        if (p == q) {
            continue;
        }

        let n = &p * &q;
        let phi = (&p - BigUint::one()) * (&q - BigUint::one());

        if bigint_gcd(&e, &phi) == BigUint::one() {
            if let Some(d) = big_int_mod_inverse(e.clone(), phi.clone()) {
                if &d < &n {
                    return (n, e, d);
                }
            }
        }
    }
}
pub fn encrypt_rsa(message: &[u8; 10], n : &BigUint, e : &BigUint) -> String {
    let m  = BigUint::from_bytes_be(message.as_ref());
    let c = m.modpow(e,n);
    hex::encode(c.to_bytes_be())
}
pub fn decrypt_rsa(ciphertext: &str, n : &BigUint, d : &BigUint) -> Vec<u8> {
    let bytes = hex::decode(ciphertext).expect("Invalid hex string");
    let c = BigUint::from_bytes_be(&bytes);
    let m = c.modpow(d,n);
    m.to_bytes_be()
}