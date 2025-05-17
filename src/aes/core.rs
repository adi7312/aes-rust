use rand::seq::IndexedRandom;

use crate::aes::modes::ctr;
use crate::aes::math::galois;

const RCON: [u8;11] = [0x0,0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36];
const SBOX: [u8;256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16
];

const INV_SBOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

// Strategy pattern for AES encryption and decryption
// It allows better support for different modes of AES
pub trait AesMode {
    
    fn encrypt(&self, input: &[u8], expanded_key: &[[u8;4];44], nonce: Option<&[u8;8]>) -> Vec<u8>;
    fn decrypt(&self, input: &[u8], expanded_key: &[[u8;4];44]) -> Vec<u8>;

}

pub trait AesCipher: Send + Sync {
    fn encrypt(&self, input: &[u8]) -> Vec<u8>;
    fn decrypt(&self, input: &[u8]) -> Vec<u8>;
}

pub struct AES<T: AesMode>{
    mode: T,
    key: [u8; 16],
    nonce: Option<[u8;8]>
}

impl<T: AesMode> AES<T> {
    pub fn new(mode: T, key: &[u8;16], nonce: Option<&[u8;8]>) -> Self {
        AES { mode, key: *key, nonce: nonce.copied() }
    }
    pub fn encrypt(&self, input: &[u8]) -> Vec<u8> {
        self.mode.encrypt(input, &key_expansion(&self.key), self.nonce.as_ref())
    }

    pub fn decrypt(&self, input: &[u8]) -> Vec<u8> {
        self.mode.decrypt(input, &key_expansion(&self.key))
    }
}

impl<T: AesMode + Send + Sync + 'static> AesCipher for AES<T> {
    fn encrypt(&self, input: &[u8]) -> Vec<u8> {
        self.encrypt(input)
    }

    fn decrypt(&self, input: &[u8]) -> Vec<u8> {
        self.decrypt(input)
    }
}


fn rcon(round: u8) -> [u8;4]{
    let mut rcon_vec: [u8;4] = [0u8;4];
    rcon_vec[0] = RCON[round as usize];
    rcon_vec
}

fn xor(a: &[u8;4], b: &[u8;4]) -> [u8;4] {
    let mut result: [u8;4] = [0u8;4];
    for i in 0..4 {
        result[i] = a[i] ^ b[i];
    }
    result
}

fn substitute_byte(byte: u8, encryption_mode: bool) -> u8 {
    if encryption_mode {
        SBOX[byte as usize]
    } else {
        INV_SBOX[byte as usize]
    }
}

fn substitute_word(word: &[u8;4]) -> [u8;4] {
    let mut result: [u8;4] = [0u8;4];
    for i in 0..4 {
        result[i] = substitute_byte(word[i], true);
    }
    result
}

fn rotate_word(word: &[u8;4]) -> [u8;4] {
    let mut result: [u8;4] = [0u8;4];
    for i in 0..4 {
        result[i] = word[(i + 1) % 4];
    }
    result
}

fn key_expansion(key: &[u8;16]) -> [[u8;4];44] {
    let mut matrix_key: [[u8;4];4] = [[0u8;4];4];
    let mut expanded_key: [[u8;4];44] = [[0u8;4];44];
    let word_size = 4;
    for i in 0..16 {
        matrix_key[i/4][i%4] = key[i];
    }
    for i in 0..44 {
        if i < word_size {
            expanded_key[i] = matrix_key[i];
        } else if i % word_size == 0 {
            let mut temp: [u8;4] = [0u8;4];
            temp[0] = RCON[i/word_size];
            expanded_key[i] = xor(
                &xor(&expanded_key[i - 1],&substitute_word(&rotate_word(&expanded_key[i - 1]))),
                &temp
            );
        } else {
            expanded_key[i] = xor(&expanded_key[i - 1], &expanded_key[i - word_size]);
        }
    }
    expanded_key
}

fn sub_block(block: &mut [[u8;4];4], encryption_mode: bool){
    for i in 0..4 {
        for j in 0..4 {
            block[i][j] = substitute_byte(block[i][j], encryption_mode);
        }
    }
}

fn add_round_key(block: &mut[[u8;4];4], round_key: &[[u8;4];4]){
    for i in 0..4 {
        for j in 0..4 {
            block[i][j] = block[i][j] ^ round_key[i][j];
        }
    }
}

fn shift_rows(block: &mut[[u8;4];4]){
    
    for i in 1..4{
        let mut tmp = vec![0u8; i as usize];
        for j in 0..i {
            tmp[j as usize] = block[i as usize][j as usize];
        }
        for j in 0..4-i{
            block[i as usize][j as usize] = block[i as usize][(j + i) as usize];
        }
        for j in 0..i{
            block[i as usize][(3-j) as usize] = tmp[(i-j-1) as usize];
        }
    }
}

fn inv_shift_rows(block: &mut[[u8;4];4]){
    for i in (1..4).rev(){
        let mut tmp = vec![0u8; i as usize];
        for j in 0..i {
            tmp[j as usize] = block[(4-i) as usize][j as usize];
        }
        for j in 0..4-i{
            block[(4-i) as usize][(j) as usize] = block[(4-i) as usize][(j + i) as usize];
        }
        for j in 0..i{
            block[(4-i) as usize][(3-j) as usize] = tmp[(i-j-1) as usize];
        }
    }
}

fn mix_columns(block: &mut[[u8;4];4]){
    for i in 0..4 {
        let mut col = [0u8;4];
        for j in 0..4 {
            col[j] = block[j][i];
        }
        block[0][i] = galois::galois_multiply(col[0], 2) ^ galois::galois_multiply(col[1], 3) ^ col[3] ^ col[2];
        block[1][i] = col[0] ^ galois::galois_multiply(col[1], 2) ^ galois::galois_multiply(col[2], 3) ^ col[3];
        block[2][i] = col[0] ^ col[1] ^ galois::galois_multiply(col[2], 2) ^ galois::galois_multiply(col[3], 3);
        block[3][i] = galois::galois_multiply(col[0], 3) ^ col[1] ^ col[2] ^ galois::galois_multiply(col[3], 2);
    }
}

fn inv_mix_columns(block: &mut[[u8;4];4]){
    for i in 0..4 {
        let mut col = [0u8;4];
        for j in 0..4 {
            col[j] = block[j][i];
        }
        block[0][i] = galois::galois_multiply(col[0], 14) ^ galois::galois_multiply(col[1], 11) ^ galois::galois_multiply(col[2], 13) ^ galois::galois_multiply(col[3], 9);
        block[1][i] = galois::galois_multiply(col[0], 9) ^ galois::galois_multiply(col[1], 14) ^ galois::galois_multiply(col[2], 11) ^ galois::galois_multiply(col[3], 13);
        block[2][i] = galois::galois_multiply(col[0], 13) ^ galois::galois_multiply(col[1], 9) ^ galois::galois_multiply(col[2], 14) ^ galois::galois_multiply(col[3], 11);
        block[3][i] = galois::galois_multiply(col[0], 11) ^ galois::galois_multiply(col[1], 13) ^ galois::galois_multiply(col[2], 9) ^ galois::galois_multiply(col[3], 14);
    }
}

pub fn pkcs7_pad(input: &[u8], block_size: usize) -> Vec<u8> {
    let pad_len = block_size - (input.len() % block_size);
    let mut padded = Vec::from(input);
    padded.extend(vec![pad_len as u8; pad_len]);
    padded
}


pub fn pkcs7_unpad(input: &[u8]) -> Vec<u8> {
    let pad_len = *input.last().unwrap() as usize;
    let len = input.len();
    if pad_len == 0 || pad_len > len {
        panic!("Invalid padding");
    }
    input[..len - pad_len].to_vec()
}

pub fn encrypt_block(input: &[u8; 16], expanded_key: &[[u8; 4]; 44]) -> [u8; 16] {
    let mut result = [0u8; 16];
    let mut input_matrix = [[0u8; 4]; 4];
    for i in 0..16 {
        input_matrix[i % 4][i / 4] = input[i];
    }
    let round_key: &[[u8;4];4] = &expanded_key[40..44].try_into().expect("Failed to convert expanded key to 4x4 matrix");
    add_round_key(&mut input_matrix, &round_key);
    for i in 1..10 {
        sub_block(&mut input_matrix, true);
        shift_rows(&mut input_matrix);
        mix_columns(&mut input_matrix);
        let round_key: &[[u8;4];4] = &expanded_key[(i*4)..((i+1)*4)].try_into().expect("Failed to convert expanded key to 4x4 matrix");
        add_round_key(&mut input_matrix, &round_key);
    }
    sub_block(&mut input_matrix, true);
    shift_rows(&mut input_matrix);
    let round_key: &[[u8;4];4] = &expanded_key[40..44].try_into().expect("Failed to convert expanded key to 4x4 matrix");
    add_round_key(&mut input_matrix, &round_key);
    for i in 0..16 {
        result[i] = input_matrix[i % 4][i / 4];
    }
    result
}

pub fn decrypt_block(input: &[u8;16], expanded_key: &[[u8;4];44]) -> [u8;16]{
    let mut result = [0u8; 16];
    let mut input_matrix = [[0u8; 4]; 4];
    for i in 0..16 {
        input_matrix[i % 4][i / 4] = input[i];
    }
    let round_key: &[[u8;4];4] = &expanded_key[40..44].try_into().expect("Failed to convert expanded key to 4x4 matrix");
    add_round_key(&mut input_matrix, round_key);
    inv_shift_rows(&mut input_matrix);
    inv_mix_columns(&mut input_matrix);
    for i in (1..10).rev() {
        let round_key = &expanded_key[(i*4)..((i+1)*4)].try_into().expect("Failed to convert expanded key to 4x4 matrix");
        add_round_key(&mut input_matrix, round_key);
        inv_mix_columns(&mut input_matrix);
        inv_shift_rows(&mut input_matrix);
        sub_block(&mut input_matrix, false);
    }
    for i in 0..16 {
        result[i] = input_matrix[i % 4][i / 4];
    }
    result
}