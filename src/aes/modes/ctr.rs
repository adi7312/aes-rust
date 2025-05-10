use crate::aes::core::{AesMode};

pub struct AesCtr {
    expanded_key: [[u8; 4]; 44],
    counter: u8,
}

impl AesMode for AesCtr {
    fn encrypt(&self, input: &[u8; 16], key: &[u8; 16]) -> [u8; 16] {
        let mut output = [0u8; 16];
        let mut counter_block = self.counter.to_le_bytes();
        for i in 0..16 {
            counter_block[i % 4] ^= key[i % 4];
            output[i] = input[i] ^ counter_block[i % 4];
        }
        output
    }

    fn decrypt(&self, input: &[u8; 16], key: &[u8; 16]) -> [u8; 16] {
        self.encrypt(input, key)
    }
}