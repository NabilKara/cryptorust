
pub fn add_blocks(state: &mut [u8; 16], b: &[u8]){
    for i in 0..16{
        state[i] ^= b[i];
    }
}
pub fn gal_mul(a: u8, b: u8) -> u8{
    let mut result : u8 = 0;
    let mut a = a;
    let mut b = b;
    const IRREDUCIBLE_POLY: u8 = 0x1b; // which corresponds to :  x^8 + x ^ 4 + x ^ 3 + x + 1
    while b != 0 {
        // if the LSB is set , we add 'a' to the result
        if (b & 1) != 0 {
            result ^= a;
        }
        // check if the MSB is set to perform modular reduction
        let msb_set = (a & 0x80) != 0;

        a <<= 1; // left shift i.e mult by x

        if msb_set{
            a ^= IRREDUCIBLE_POLY;
        }
        b >>= 1; // right shift i.e division to move to the next bit
    }
    result
}