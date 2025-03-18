#[cfg(test)]
mod tests {
    use crate::symmetric_encryption::des::*;

    const HALF_SIZE: usize = 4;
    const BLOCK_SIZE: usize = 8;

    #[test]
    fn test_set_bit() {
        // Test setting bit at position 1 (LSB)
        assert_eq!(setBit_64(0b0000, 1), 0b0001);

        // Test setting bit at position 4
        assert_eq!(setBit_64(0b0000, 4), 0b1000);

        // Test setting bit at position 64 (MSB for u64)
        assert_eq!(setBit_64(0b0000, 64), 0x8000000000000000);

        // Test setting a bit that is already set
        assert_eq!(setBit_64(0b0100, 3), 0b0100);
    }

    #[test]
    fn test_get_bit() {
        // Test getting bit at position 1 (LSB)
        assert_eq!(getBit_64(0b0001, 1), 0b0001);

        // Test getting bit at position 4
        assert_eq!(getBit_64(0b1000, 4), 0b1000);

        // Test getting bit at position 64 (MSB for u64)
        assert_eq!(getBit_64(0x8000000000000000, 64), 0x8000000000000000);

        // Test getting a bit that is not set
        assert_eq!(getBit_64(0b0100, 2), 0b0000);
    }

    #[test]
    fn test_convert8to32() {
        // Test converting 4 bytes to u32
        let bytes: [u8; HALF_SIZE] = [0x12, 0x34, 0x56, 0x78];
        assert_eq!(convert8to32(&bytes), 0x12345678);

        // Test with all zeros
        let zeros: [u8; HALF_SIZE] = [0x00, 0x00, 0x00, 0x00];
        assert_eq!(convert8to32(&zeros), 0x00000000);

        // Test with all ones
        let ones: [u8; HALF_SIZE] = [0xFF, 0xFF, 0xFF, 0xFF];
        assert_eq!(convert8to32(&ones), 0xFFFFFFFF);
    }

    #[test]
    fn test_convert32to8() {
        // Test converting u32 to 4 bytes
        let num: u32 = 0x12345678;
        assert_eq!(convert32to8(num), [0x12, 0x34, 0x56, 0x78]);

        // Test with all zeros
        let zeros: u32 = 0x00000000;
        assert_eq!(convert32to8(zeros), [0x00, 0x00, 0x00, 0x00]);

        // Test with all ones
        let ones: u32 = 0xFFFFFFFF;
        assert_eq!(convert32to8(ones), [0xFF, 0xFF, 0xFF, 0xFF]);
    }

    #[test]
    fn test_convert8to64() {
        // Test converting 8 bytes to u64
        let bytes: [u8; BLOCK_SIZE] = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
        assert_eq!(convert8to64(&bytes), 0x123456789ABCDEF0);

        // Test with all zeros
        let zeros: [u8; BLOCK_SIZE] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(convert8to64(&zeros), 0x0000000000000000);

        // Test with all ones
        let ones: [u8; BLOCK_SIZE] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        assert_eq!(convert8to64(&ones), 0xFFFFFFFFFFFFFFFF);
    }

    #[test]
    fn test_convert64to8() {
        // Test converting u64 to 8 bytes
        let num: u64 = 0x123456789ABCDEF0;
        assert_eq!(convert64to8(num), [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]);

        // Test with all zeros
        let zeros: u64 = 0x0000000000000000;
        assert_eq!(convert64to8(zeros), [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        // Test with all ones
        let ones: u64 = 0xFFFFFFFFFFFFFFFF;
        assert_eq!(convert64to8(ones), [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    }
}
