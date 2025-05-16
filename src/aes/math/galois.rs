pub fn galois_multiply(a: u8, b: u8) -> u8 {
    let mut result = 0u8;
    let mut tmp_a = a;
    let mut tmp_b = b;
    let mut high_bit = 0u8;
    for _ in 0..8 {
        if tmp_b & 1 != 0 {
            result ^= tmp_a;
        }
        high_bit = tmp_a & 0x80;
        tmp_a <<= 1;
        if high_bit != 0 {
            tmp_a ^= 0x1b; // x^8 + x^4 + x^3 + x + 1
        }
        tmp_b >>= 1;
    }
    result
}