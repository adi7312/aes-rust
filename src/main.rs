pub mod aes;
use aes::core::{AES};
use aes::modes::ctr::CTR;
fn main() {
    let key = [0u8; 16];
    let mut plaintext = [0u8; 320];
    for i in 0..320 {
        plaintext[i] = (i%256) as u8;
    }
    let aes = AES::new(CTR,&key);
    println!("Plaintext: {:?}", plaintext);
    let ciphertext = aes.encrypt(&plaintext);
    println!("Ciphertext: {:?}", ciphertext);
    let decrypted = aes.decrypt(&ciphertext);
    println!("Plaintext restored: {:?}", decrypted);
}