use std::sync::{Arc, Mutex};
use crate::aes::core::{AesMode, encrypt_block};
use rand::prelude::*;
use std::thread;
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
        
        let num_chunks = input.len() / 16;
        let threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
            .min(num_chunks);
        let result_blocks = Arc::new(Mutex::new(vec![vec![0u8; 16]; num_chunks]));
        
        let blocks_per_thread = (num_chunks + threads - 1) / threads;
        
        let key = Arc::new(*expanded_key);
        let input_arc = Arc::new(input.to_vec());
        let nonce = counter[..8].to_vec();
        
        let mut handles = Vec::with_capacity(threads);
        for t in 0..threads {
            let key = Arc::clone(&key);
            let input = Arc::clone(&input_arc);
            let result_blocks = Arc::clone(&result_blocks);
            let nonce = nonce.clone();
            
            let start_block = t * blocks_per_thread;
            let end_block = ((t + 1) * blocks_per_thread).min(num_chunks);
            
            if start_block >= end_block{
                continue;
            }
            
            let handle = thread::spawn(move || {
                for block_index in start_block..end_block {
                    let mut counter = [0u8; 16];
                    counter[..8].copy_from_slice(&nonce);
                    counter[8..].copy_from_slice(&(block_index as u64).to_le_bytes());

                    let keystream = encrypt_block(&counter, &key);

                    let start = block_index * 16;
                    let chunk = &input[start..start + 16];

                    let mut encrypted_block = [0u8; 16];
                    for i in 0..16 {
                        encrypted_block[i] = chunk[i] ^ keystream[i];
                    }

                    result_blocks.lock().unwrap()[block_index] = encrypted_block.to_vec();
                }
            });
            handles.push(handle);
            
        }
        for handle in handles{
            handle.join().expect("thread panicked during AES CTR encryption");
        }
        let final_blocks = result_blocks.lock().unwrap();
        for block in final_blocks.iter(){
            result.extend_from_slice(block);
        }
        result
        // multi-threaded encryption IMPLEMENT HERE
        
        
        // NORMAL ENCRYPTION
        // for chunk in input.chunks(16) {
        //     let encrypted_counter = encrypt_block(&counter, expanded_key);
        //     
        //     let mut block = [0u8; 16];
        //     for i in 0..16 {
        //         block[i] = chunk[i] ^ encrypted_counter[i];
        //     }
        //     result.extend_from_slice(&block);
        //     
        //     counter_value = counter_value.wrapping_add(1);
        //     counter[8..].copy_from_slice(&counter_value.to_le_bytes());
        // }
        
        
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