#[cfg(test)]
mod tests {
    use crate::asymmetric_encryption::RSA::rsa_generate_key_pair;
    #[test]
    fn generate_key_pair(){
        let (n,e,d) = rsa_generate_key_pair(1024, 10);
        println!("n: {}",n);
        println!("e: {}",e);
        println!("d: {}",d);
    }
}