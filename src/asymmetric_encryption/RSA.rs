use crate::asymmetric_encryption::utils::{big_int_mod_inverse, bigint_gcd, generate_safe_prime};
use num_bigint::BigUint;
use num_traits::One;

const MAX_ATTEMPTS: usize = 100;

pub fn rsa_generate_key_pair(bits: u64, rounds: usize) -> (BigUint, BigUint, BigUint) {
    let e = BigUint::from(65537u32);
    let mut attempts = 0;
    loop {
        attempts += 1;
        if attempts > MAX_ATTEMPTS {
            panic!("Failed to generate RSA key pair after {} attemps.", MAX_ATTEMPTS);
        }
        let p = generate_safe_prime(bits / 2, rounds);
        let q = generate_safe_prime(bits / 2, rounds);

        if (p == q) {
            continue;
        }

        let n = &p * &q;
        let phi = (&p - BigUint::one()) * (&q - BigUint::one());
        println!("p:     {}", p);
        println!("q:     {}", q);
        println!("p * q: {}", n);
        println!("phi:   {}", phi);
        
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