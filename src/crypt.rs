use super::*;

pub static F_TABLE: [u8; 256] = [
    0xa3, 0xd7, 0x09, 0x83, 0xf8, 0x48, 0xf6, 0xf4, 0xb3, 0x21, 0x15, 0x78, 0x99, 0xb1, 0xaf, 0xf9, 
    0xe7, 0x2d, 0x4d, 0x8a, 0xce, 0x4c, 0xca, 0x2e, 0x52, 0x95, 0xd9, 0x1e, 0x4e, 0x38, 0x44, 0x28, 
    0x0a, 0xdf, 0x02, 0xa0, 0x17, 0xf1, 0x60, 0x68, 0x12, 0xb7, 0x7a, 0xc3, 0xe9, 0xfa, 0x3d, 0x53, 
    0x96, 0x84, 0x6b, 0xba, 0xf2, 0x63, 0x9a, 0x19, 0x7c, 0xae, 0xe5, 0xf5, 0xf7, 0x16, 0x6a, 0xa2, 
    0x39, 0xb6, 0x7b, 0x0f, 0xc1, 0x93, 0x81, 0x1b, 0xee, 0xb4, 0x1a, 0xea, 0xd0, 0x91, 0x2f, 0xb8, 
    0x55, 0xb9, 0xda, 0x85, 0x3f, 0x41, 0xbf, 0xe0, 0x5a, 0x58, 0x80, 0x5f, 0x66, 0x0b, 0xd8, 0x90, 
    0x35, 0xd5, 0xc0, 0xa7, 0x33, 0x06, 0x65, 0x69, 0x45, 0x00, 0x94, 0x56, 0x6d, 0x98, 0x9b, 0x76, 
    0x97, 0xfc, 0xb2, 0xc2, 0xb0, 0xfe, 0xdb, 0x20, 0xe1, 0xeb, 0xd6, 0xe4, 0xdd, 0x47, 0x4a, 0x1d, 
    0x42, 0xed, 0x9e, 0x6e, 0x49, 0x3c, 0xcd, 0x43, 0x27, 0xd2, 0x07, 0xd4, 0xde, 0xc7, 0x67, 0x18, 
    0x89, 0xcb, 0x30, 0x1f, 0x8d, 0xc6, 0x8f, 0xaa, 0xc8, 0x74, 0xdc, 0xc9, 0x5d, 0x5c, 0x31, 0xa4, 
    0x70, 0x88, 0x61, 0x2c, 0x9f, 0x0d, 0x2b, 0x87, 0x50, 0x82, 0x54, 0x64, 0x26, 0x7d, 0x03, 0x40, 
    0x34, 0x4b, 0x1c, 0x73, 0xd1, 0xc4, 0xfd, 0x3b, 0xcc, 0xfb, 0x7f, 0xab, 0xe6, 0x3e, 0x5b, 0xa5, 
    0xad, 0x04, 0x23, 0x9c, 0x14, 0x51, 0x22, 0xf0, 0x29, 0x79, 0x71, 0x7e, 0xff, 0x8c, 0x0e, 0xe2, 
    0x0c, 0xef, 0xbc, 0x72, 0x75, 0x6f, 0x37, 0xa1, 0xec, 0xd3, 0x8e, 0x62, 0x8b, 0x86, 0x10, 0xe8, 
    0x08, 0x77, 0x11, 0xbe, 0x92, 0x4f, 0x24, 0xc5, 0x32, 0x36, 0x9d, 0xcf, 0xf3, 0xa6, 0xbb, 0xac, 
    0x5e, 0x6c, 0xa9, 0x13, 0x57, 0x25, 0xb5, 0xe3, 0xbd, 0xa8, 0x3a, 0x01, 0x05, 0x59, 0x2a, 0x46
];

pub const ROW_LEN: usize = 16;
pub const COL_LEN: usize = 12;
pub const NUM_ROUNDS: usize = 16;

/// # Description
/// Takes an unsigned 8-bit integer as an index to return the corresponding value of that index within the F Table
/// The high 4 bits are used to index the row
/// The low 4 bits are used to index the column
///
/// # Arguments
/// * `index` - An unsigned 8-bit integer
///
/// # Example
/// let value = crypt::get_f_table_value(0x7a);
/// assert!(value == 0xd6);
pub fn get_f_table_value(index: u8) -> u8 {
    let row = (index >> 4) as usize;
    // I hope there's a more elegant way to find a column
    let col = (index.rotate_right(4) >> 4) as usize;

    F_TABLE[row*ROW_LEN + col]
}

/// # Description
/// Retrieves values from the F table and performs bitwise xors to obtain a final unsigned 16-bit integer to be used in the F function
///
/// # Arguments
/// * `r` - An unsigned 16-bit integer. The function breaks up the low 8 bits from the high 8 bits in order to get values from the F table
/// * `subkeys` - A slice of a vector of unsigned 8-bit integers. Used to help get values from the F table.
/// * `_round` - A pointer-sized integer. This value does not actually serve any purpose in this function.
///
/// # Example
/// let r0 = 0xaaee;
/// let subkeys = vec![0x13, 0x9e, 0x2b, 0x34, 0x35, 0xe2,0xb3, 0x45, 0x57, 0x26, 0x3c, 0x56];
/// let t0 = crypt::g(r0, &subkeys[0..4], 0);
/// assert!(t0 == 0xf889);
pub fn g(r: u16, subkeys: &[u8], _round: usize) -> u16 {
    let g1 = (r >> 8) as u8;
    let g2 = r as u8;
    
    let g3 = get_f_table_value(g2 ^ subkeys[0]) ^ g1;
    let g4 = get_f_table_value(g3 ^ subkeys[1]) ^ g2;
    let g5 = get_f_table_value(g4 ^ subkeys[2]) ^ g3;
    let g6 = get_f_table_value(g5 ^ subkeys[3]) ^ g4;

    convert_types::to_u16_block(&g5, &g6)
}

/// # Description
/// Makes calls to the G function and performs summations to return a tuple of two unsigned 16-bit integers, which are used for creating key blocks each round
///
/// # Arguments
/// * `r0` - An unsigned 16-bit integer. This value is one of the blocks obtained after input whitening.
/// * `r1` - An unsigned 16-bit integer. This value is one of the blocks obtained after input whitening.
/// * `subkeys` - A slice of a vector of unsigned 8-bit integers. Used to help get values from the F table and perform summations.
/// * `_round` - A pointer-sized integer. This value does not actually serve any purpose in this function.
///
/// # Example
/// let mut r0 = 0xaaee;
/// let mut r1 = 0xaa66;
/// let subkeys = vec![0x13, 0x9e, 0x2b, 0x34, 0x35, 0xe2,0xb3, 0x45, 0x57, 0x26, 0x3c, 0x56];
/// let (f0, f1) = crypt::f(r0, r1, &subkeys, 0);
/// assert!(f0 == 0x3eb1);
/// assert!(f1 == 0xa4e9);
pub fn f(r0: u16, r1: u16, subkeys: &Vec<u8>, round: usize) -> (u16, u16) {
    let t0 = g(r0, &subkeys[0..4], round);
    let t1 = g(r1, &subkeys[4..8], round);

    let sum = t0 as u32 + 2*t1 as u32 + convert_types::to_u16_block(&subkeys[8], &subkeys[9]) as u32; 
    let f0 = (sum % 2u32.pow(16)) as u16;

    let sum = 2*t0 as u32 + t1 as u32 + convert_types::to_u16_block(&subkeys[10], &subkeys[11]) as u32;
    let f1 = (sum % 2u32.pow(16)) as u16;

    (f0, f1)
}

/// # Description
/// Accepts two vectors of bytes as input (key and plaintext) and returns an encrypted unsigned 64-bit integer representing the ciphertext
///
/// # Arguments
/// * `key` - A vector of bytes. This value is obtained from the file 'input/key.txt'
/// * `plaintext` - A vector of bytes. This value is obtained from the file 'input/plaintext.txt'
///
/// # Example
/// let key: Vec<u8> = vec![0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89];
/// let plaintext: Vec<u8> = vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
/// 
/// let ciphertext = crypt::encrypt(&key, &plaintext);
/// 
/// assert!(ciphertext == 0x2ad9c6e5b8fe56fb);
pub fn encrypt(key: &Vec<u8>, plaintext: &Vec<u8>) -> u64 {
    let results = whiten::whiten_input(key, plaintext);

    let mut r0 = results[0];
    let mut r1 = results[1];
    let mut r2 = results[2];
    let mut r3 = results[3];

    let mut key_block = convert_types::create_key_block(&key);

    for r in 0..NUM_ROUNDS {
        let subkeys = subkey_gen::generate_subkeys_for_encrypt(&mut key_block, r);
        let (f0, f1) = f(r0, r1, &subkeys, r);

        let temp_r2 = r2;
        let temp_r3 = r3;
        r2 = r0;
        r3 = r1;
        r0 = temp_r2 ^ f0 as u16;
        r1 = temp_r3 ^ f1 as u16;
    }

    let y = vec![r2, r3, r0, r1];

    whiten::whiten_output(&whiten::create_whitening_blocks(&key), &y)
}

/// # Description
/// Accepts two vectors of bytes as input (key and ciphertext) and returns a decrypted unsigned 64-bit integer representing the plaintext
///
/// # Arguments
/// * `key` - A vector of bytes. This value is obtained from the file 'input/key.txt'
/// * `ciphertext` - A vector of bytes. This value is obtained from the file 'input/ciphertext.txt'
///
/// # Example
/// let key: Vec<u8> = vec![0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89];
/// let ciphertext: Vec<u8> = vec![0x2a, 0xd9, 0xc6, 0xe5, 0xb8, 0xfe, 0x56, 0xfb];
/// 
/// let plaintext = crypt::decrypt(&key, &ciphertext);
/// 
/// assert!(plaintext == 0x01234567890abcdef);
pub fn decrypt(key: &Vec<u8>, ciphertext: &Vec<u8>) -> u64 {
    let results = whiten::whiten_input(key, ciphertext);

    let mut r0 = results[0];
    let mut r1 = results[1];
    let mut r2 = results[2];
    let mut r3 = results[3];

    let mut key_block = convert_types::create_key_block(&key);

    for r in (0..NUM_ROUNDS).rev() {
        let subkeys = subkey_gen::generate_subkeys_for_decrypt(&mut key_block, r);
        let (f0, f1) = f(r0, r1, &subkeys, r);

        let temp_r2 = r2;
        let temp_r3 = r3;
        r2 = r0;
        r3 = r1;
        r0 = temp_r2 ^ f0 as u16;
        r1 = temp_r3 ^ f1 as u16;
    }

    let y = vec![r2, r3, r0, r1];

    whiten::whiten_output(&whiten::create_whitening_blocks(&key), &y)
}
