use std::ops::Sub;
use num_bigint::{BigInt, BigUint};
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