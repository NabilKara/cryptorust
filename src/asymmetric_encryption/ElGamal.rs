use num_bigint::{BigUint, RandBigInt};
use rand::thread_rng;
use crate::asymmetric_encryption::utils::generate_safe_prime;
pub fn ElGamal_generate_keys(bit_size: u64, rounds : usize) -> (BigUint, BigUint,BigUint, BigUint) {
  let p = generate_safe_prime(bit_size,rounds);
  let mut rng = thread_rng();
  let g = BigUint::from(2u32);
  let private_key  = rng.gen_biguint_range(&BigUint::from(2u32), &(&p - &BigUint::from(2u32)));
  let public_key = g.modpow(&private_key, &p);
  (public_key, private_key,p, g)
}
pub fn ElGamal_encrypt(message : &BigUint, public_key: &BigUint, p : &BigUint, g : &BigUint) -> (BigUint, BigUint) {

  let mut rng = thread_rng();
  let i  = rng.gen_biguint_range(&BigUint::from(2u32), &(p - &BigUint::from(2u32)));
  let ephemeral_public_key = g.modpow(&i, p);
  let masking_public_key = public_key.modpow(&i, p);
  let ciphertext = (message * &masking_public_key) % p;
  (ciphertext, ephemeral_public_key)
}
pub fn ElGamal_decrypt(ciphertext: &BigUint, ephemeral_public_key: &BigUint, p: &BigUint, private_key: &BigUint) -> BigUint {
  let masking_public_key = ephemeral_public_key.modpow(private_key, p);
  let inverse_masking_public_key = masking_public_key.modinv(p).unwrap();
  (ciphertext * inverse_masking_public_key) % p
}