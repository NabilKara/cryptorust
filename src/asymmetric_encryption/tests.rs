#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use crate::asymmetric_encryption::RSA::{decrypt_rsa, encrypt_rsa, rsa_generate_key_pair};
    use crate::asymmetric_encryption::DH_key_exchange::{DH_generate_key_pair,DH_shared_secret,MODP_2048};
        #[test]
        fn generate_key_pair(){
            let (n,e,d) = rsa_generate_key_pair(512, 5);
            println!("n: {}",n);
            println!("e: {}",e);
            println!("d: {}",d);
       }
        #[test]
        fn test_rsa_encrypt_decrypt() {

            let n_hex = "b64796d27939ea105c63b9fbd38520733d02e31cd1ab0ef31a818fe64fa41f24a2b5b31c997deca96fcd65eda55386798041f02e6f4d501b92a8726fc307d38a821cd5222dc262a58794f4d178466c9bee6b988523f00cf49603e9a88ebf5cd6bfd9c3c6af5c4dea7a7c6ee30f199a534a4328359d63b08a8aadf7dc881532b249c168d1351e7fc4fcdd49879f533ec036f5a75d6956d99792592cde84a8bf97edf993761b4f238475f47660c8de95fbaaf4744136300720aa8936d28f4810fbb198b9aab3ffabda0419cf0ca9923d53d72f0b6054a9b0e56d7bd814ea8ac3f77f4ad1d48efd48bddc7343f01d6221ae62dc88186450b4100f74cdf549f34cc5a494fb917503e7ffbbf80033899edabcfc767d7c800a20e0dce61b777f0d9cf64fcc36c078a885d4f4a951326ec39dd5b96037aca43f71284cdfe5dde209b1d98c14caa7c570e4b921a678de1ecc8687758f71a604e17b10695820d7213f90c954ac64cedcd4b8d755d4aa400c5c7e400777bea2d569c326a7853fd14ed46b8dcf231443b00984c466bdf98ef4a44f46f32fc209aab66f82f02c3596def4fcb8ba2879bc51aa471ebdef7ceac40eef84dd07f5ece3269b113d645920bdc1a5607429b496d609bee980cc746d494bdd1e5a41e1a3b4975b8dfea4763f3afe277468ad9cfc5cd882650f3d19c4a15b99dfa385344e99ac1847cccc090e043ac8";
            let d_hex = "a91ac1518225b88075360fd510e6f0b3f866354d07372b275defd51055d26d120fe181c9869ccc7264c6a9494f97adaa0ad3217ff2c2470f4d7ff8aab6c50bfbf3b9cc0169726d1224368ce9c82b8d971c19b5e5cef1651384191d2cc341c55f759d1183c22c601ca885c859de286283e2e16aec5297a9b0ddf7877b90c419c44da8a11575b255d6eac060f6f158c95e899e5d26cc1346cad77ae3a58c6d015271f4985979fc7176f1f759db707f0fa5eb73de66a15eed3de0f86b63ec896f38120b1e34bfc8f225d0a820ee1e630d823a9fea6df41d797f5f93056462335a9ac1fd27ad8afa7001b4033b8407be8280c636d20bcf051007d64f06c409572e60983ccf0f9d0102797a01b52e2619e818f661d34ba6cace09acc60f5e34d7445a02f1e6583dea51289b1be24cf406812c3cdfb0bd3d1de760f7f5c3fc6fb499dca0375f072db2a9a69f2e20caaab2e97daf919540e88570a697d915f954cde7884574c512eb40b92289a6c0a2b8ffab4e7c87e15c863665510b63a79c2f7f44b622f410ef2678584a863692ad4af5065029c15d37b2579d35272c6806d329f026adc445aa60cb264fd36a4c37d1d4af55a994af168f7095e8f328fd554db72c70142e32cd8daf1f4b8a79ad7dfb59ce650ac04faa6ab3b27b20b36e96995bf2f5b900f9d58833152b26c00df1ab194342b0427ce8f4089022dd0b18e5832ae9";

            let n = BigUint::parse_bytes(n_hex.as_bytes(), 16).unwrap();
            let d = BigUint::parse_bytes(d_hex.as_bytes(), 16).unwrap();
            let e = BigUint::from(65537u32);

            let message = b"Hello RSA!";

            let ciphertext = encrypt_rsa(message, &e, &n);
            let decrypted = decrypt_rsa(&ciphertext, &d, &n);

            assert_eq!(message, decrypted.as_slice());
        }
        #[test]
        fn test_DH_key_exchange() {
            let g = BigUint::from(2u32);
            let (alice_private , alice_public)  = DH_generate_key_pair(&*MODP_2048, &g);
            let (bob_private , bob_public)  = DH_generate_key_pair(&*MODP_2048,&g);
            let alice_shared_secret = DH_shared_secret(&alice_private, &bob_public, &*MODP_2048);
            let bob_shared_secret = DH_shared_secret(&bob_private, &alice_public, &*MODP_2048);
            assert_eq!(alice_shared_secret, bob_shared_secret, "Shared secrets should be the same");
        }

}