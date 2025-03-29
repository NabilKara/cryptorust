use std::ops::Sub;
use lazy_static::lazy_static;
use miller_rabin::is_prime;
use num_bigint::{BigInt, BigUint, RandBigInt};
use num_traits::{One, Zero};
 pub fn bigint_gcd(A: &BigUint, B: &BigUint) -> BigUint {
    if B.is_zero() {
        B.clone()
    }else {
        bigint_gcd(B, & (A % B))
    }
 }
fn big_int_extended_gcd(A: BigUint, B: BigUint) -> (BigUint,BigUint, BigUint) {
     if (A.eq(&BigUint::ZERO)) {
        return (B.clone(), BigUint::ZERO, BigUint::one())
     }
     let (g,x1,y1) = big_int_extended_gcd((B.clone() % &A), A.clone());
     let x = y1 - (B / A) * &x1;
     let y = x1;
     (g,x,y)
 }
pub fn big_int_mod_inverse(A: BigUint, M: BigUint) -> Option<BigUint> {
    let (g,x,_) = big_int_extended_gcd(A.clone(), M.clone());
    if (g != BigUint::one()) {
        None
    }else{
        Some((x % M.clone() + M.clone()) % M.clone())

    }
}
lazy_static!{
  pub static ref SECURE_PRIME: BigUint = BigUint::parse_bytes(b"6942120798175882080594457812669318587080243130900436510867221614039874562649870591346956996743843800335323156501717076284789981677900515939965331468577627", 10).unwrap();
    pub static ref  ANOTHER_SECURE_PRIME: BigUint  = BigUint::parse_bytes(b"13352635493652824167715215077523564732081557266175750779626347670024901835121453931635721885617436238301897405897613394213979133513060312480692170686761343",10).unwrap();
}

pub fn generate_safe_prime(bit_size: u64, rounds: usize) -> BigUint {
    let max_attempts = match bit_size {
        512 => 500_000,
        1024 => 2_000_000,
        2048 => 10_000_000,
        3072 => 30_000_000,
        4096 => 100_000_000,
        _ => 10_000_000,
    };
    let mut rng = rand::thread_rng();
    let mut attempts = 0;
    loop {
        if attempts == max_attempts {
            panic!("failed to generate a safe prime after {} attempts", max_attempts)
        }
        let mut q = rng.gen_biguint(bit_size - 1);
        q.set_bit(bit_size - 2, true);// Ensure bit length
        q.set_bit(0, true); // Make odd

        if !is_prime(&q, rounds) {
            attempts += 1;
            continue;
        }

        // check if 2q + 1 is prime
        let p = &q * BigUint::from(2u32) + BigUint::one();

        if is_prime(&p, rounds) {
            return p;
        }
        attempts += 1;
        if attempts % 1000 == 0 {
            println!("passed {} attempts", attempts);
        }
    }
}