use std::ops::Sub;
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};
 pub fn bigint_gcd(A: &BigUint, B: &BigUint) -> BigUint {
    if A.eq(B) {
        A.clone()
    } else if A.gt(B) {
        bigint_gcd(&(A.sub(B)), B)
    } else {
        bigint_gcd(A, &(B.sub(A)))
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
    let (g,x,y) = big_int_extended_gcd(A.clone(), M.clone());
    if (!g.eq(&BigUint::one())) {
        None
    }else{
        let res = (x % M.clone() + M.clone()) % M.clone();
        Some(res)
    }
}