use num::Integer;
use crate::asymmetric_encryption::utils::{mod_inverse, BigUint_GCD, generate_safe_prime};
use num_bigint::BigUint;
use num_traits::One;

const MAX_ATTEMPTS: usize = 100;
pub fn rsa_generate_key_pair(bits: u64, rounds: usize) -> (BigUint, BigUint, BigUint) {
    let e = BigUint::from(65537u32);
    let mut attempts = 0;
    loop {
        attempts += 1;
        if attempts > MAX_ATTEMPTS {
            panic!("Failed to generate RSA key pair after {} attempts.", MAX_ATTEMPTS);
        }
        let p = generate_safe_prime(bits / 2, rounds);
        let q = generate_safe_prime(bits / 2, rounds);

        if p == q {
            continue;
        }

        let n = &p * &q;
        let phi = (&p - BigUint::one()) * (&q - BigUint::one());
        println!("p:     {}", p);
        println!("q:     {}", q);
        println!("p * q: {}", n);
        println!("phi:   {}", phi);
        
        if BigUint_GCD(e.clone(), phi.clone()) == BigUint::one() {
            if let Some(d) = mod_inverse(e.clone(), phi.clone()) {
                if &d < &n {
                    return (n, e, d);
                }
            }
        }
    }
}

pub fn encrypt_rsa(cleartext: &Vec<u8>, e: &BigUint, n: &BigUint) -> Vec<u8> {
    println!("M: {:?}", cleartext);

    let m = BigUint::from_bytes_be(cleartext);
    println!("m: 0x{:X}", m);
    
    
    if m > *n { panic!("m '0x{m:X}' is bigger than modulus n '0x{n:X}'."); }
    let c = m.modpow(&e, &n);
    println!("c: 0x{:X}", c);

    c.to_bytes_be().to_vec()
}

pub fn decrypt_rsa(ciphertext: &Vec<u8>, d: &BigUint, n: &BigUint) -> Vec<u8> {
    println!("ciphertext: {:?}", ciphertext);

    let c = BigUint::from_bytes_be(ciphertext);
    println!("c: 0x{:X}", c);

    let m = c.modpow(&d, &n);
    println!("m: 0x{:X}", m);
    m.to_bytes_be().to_vec()
}
// pub fn encrypt_rsa(message: &[u8], n : &BigUint, e : &BigUint) -> String {
//     let m  = BigUint::from_bytes_be(message.as_ref());
//     let c = m.modpow(e,n);
//     hex::encode(c.to_bytes_be())
// }

// pub fn decrypt_rsa(ciphertext: &str, n : &BigUint, d : &BigUint) -> Vec<u8> {
//     let bytes = hex::decode(ciphertext).expect("Invalid hex string");
//     let c = BigUint::from_bytes_be(&bytes);
//     let m = c.modpow(d,n);
//     m.to_bytes_be()
// }