use crate::aes::core::{AesMode, encrypt_block};
use rand::prelude::*;
pub struct CTR;

impl AesMode for CTR {
    
    fn encrypt(&self, input: &[u8], expanded_key: &[[u8;4];44]) -> Vec<u8> {
        if input.len() % 16 != 0 {
            panic!("Input length must be a multiple of 16 bytes");
        }
        // Init counter
        let mut result = Vec::new();
        let mut counter = [0u8;16];
        let mut rng = rand::rng();
        rng.fill_bytes(&mut counter[..8]);
        let mut counter_value: u64 = 0;
        counter[8..].copy_from_slice(&counter_value.to_le_bytes());
        result.extend_from_slice(&counter[..8]);
        for chunk in input.chunks(16) {
            let encrypted_counter = encrypt_block(&counter, expanded_key);
            
            let mut block = [0u8; 16];
            for i in 0..16 {
                block[i] = chunk[i] ^ encrypted_counter[i];
            }
            result.extend_from_slice(&block);
            
            counter_value = counter_value.wrapping_add(1);
            counter[8..].copy_from_slice(&counter_value.to_le_bytes());
        }
        result
        
    }

    fn decrypt(&self, input: &[u8], expanded_key: &[[u8;4];44]) -> Vec<u8> {
        let mut result = Vec::new();
        let mut counter = [0u8;16];
        let mut counter_value: u64 = 0;
        counter[..8].copy_from_slice(&input[..8]);
        for chunk in input[8..].chunks(16){
            let encrypted_counter = encrypt_block(&counter, expanded_key);
            let mut block = [0u8;16];
            for i in 0..16 {
                block[i] = chunk[i] ^ encrypted_counter[i];
            }
            result.extend_from_slice(&block);
            counter_value = counter_value.wrapping_add(1);
            counter[8..].copy_from_slice(&counter_value.to_le_bytes());
        }
        result
    }
    
}