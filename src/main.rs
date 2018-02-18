#[allow(dead_code)]
static F_TABLE: [u8; 256] = [
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

const ROW_LEN: usize = 16;
const COL_LEN: usize = 12;
const NUM_ROUNDS: usize = 16;
const SUBKEY_GEN_ROUNDS: usize = 3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn test_whitening_stage() {
        let key: Vec<u8> = vec![0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89];
        let plaintext: Vec<u8> = vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];

        let results = whiten_input(&key, &plaintext);
        assert!(results == [0xaaee, 0xaa66, 0xaaee, 0xaa66]);
    }

    #[test]
    // #[ignore]
    fn test_g_function() {
        let r0 = 0xaaee;
        let subkeys = vec![
            0x13, 0x9e, 0x2b, 0x34, 0x35, 0xe2,
            0xb3, 0x45, 0x57, 0x26, 0x3c, 0x56
        ];
        let g1 = (r0 >> 8) as u8;
        assert!(g1 == 0xaa);
        let g2 = r0 as u8;
        assert!(g2 == 0xee);
        
        let g3 = get_f_table_value(g2 ^ subkeys[0]) ^ g1;
        assert!(g3 == 0xf3);
        let g4 = get_f_table_value(g3 ^ subkeys[1]) ^ g2;
        assert!(g4 == 0x76);
        let g5 = get_f_table_value(g4 ^ subkeys[2]) ^ g3;
        assert!(g5 == 0xf8);
        let g6 = get_f_table_value(g5 ^ subkeys[3]) ^ g4;
        assert!(g6 == 0x89);
    }

    #[test]
    // #[ignore]
    fn test_get_values_from_f_table() {
        let index: u8 = 0x7a;
        let row = (index >> 4) as usize;
        let col = (index.rotate_right(4) >> 4) as usize;
        assert!(row == 7);
        assert!(col == 10);
        assert!(F_TABLE[row*ROW_LEN + col] == 0xd6);

        assert!(get_f_table_value(0x7a) == 0xd6);
    }

    #[test]
    // #[ignore]
    fn test_f_function() {
        let r0 = 0xaaee;
        let r1 = 0xaa66;
        let subkeys = vec![
            0x13, 0x9e, 0x2b, 0x34, 0x35, 0xe2,
            0xb3, 0x45, 0x57, 0x26, 0x3c, 0x56
        ];

        let t0 = g(r0, &subkeys[0..4], 0);
        assert!(t0 == 0xf889);
        let t1 = g(r1, &subkeys[4..8], 0);
        assert!(t1 == 0x7781);

        let sum = t0 as u32 + 2*t1 as u32 + to_u16_block(&subkeys[8], &subkeys[9]) as u32; 
        let f0 = (sum % 2u32.pow(16)) as u16;
        assert!(f0 == 0x3eb1);

        let sum = 2*t0 as u32 + t1 as u32 + to_u16_block(&subkeys[10], &subkeys[11]) as u32;
        let f1 = (sum % 2u32.pow(16)) as u16;
        assert!(f1 == 0xa4e9);
    }

    #[test]
    // #[ignore]
    fn test_subkey_generation() {
        let expected: Vec<Vec<u8>> = vec![
            vec![0x13,  0x9e,  0x2b,  0x34,  0x35,  0xe2,  0xb3,  0x45,  0x57,  0x26,  0x3c,  0x56],
            vec![0x68,  0x48,  0x80,  0xef,  0x8a,  0x8d,  0x09,  0xf0,  0xac,  0xd1,  0x91,  0x01],
            vec![0xde,  0x37,  0x5e,  0x9a,  0xe0,  0x7b,  0xe6,  0xab,  0x02,  0xbc,  0x6f,  0xbc],
            vec![0x35,  0xe2,  0xb3,  0x45,  0x57,  0x26,  0x3c,  0x56,  0x79,  0x6a,  0xc4,  0x67],
            vec![0x8a,  0x8d,  0x09,  0xf0,  0xac,  0xd1,  0x91,  0x01,  0xcf,  0x15,  0x1a,  0x12],
            vec![0xe0,  0x7b,  0xe6,  0xab,  0x02,  0xbc,  0x6f,  0xbc,  0x24,  0xc0,  0xf7,  0xcd],
            vec![0x57,  0x26,  0x3c,  0x56,  0x79,  0x6a,  0xc4,  0x67,  0x9b,  0xaf,  0x4d,  0x78],
            vec![0xac,  0xd1,  0x91,  0x01,  0xcf,  0x15,  0x1a,  0x12,  0xf1,  0x59,  0xa2,  0x23],
            vec![0x02,  0xbc,  0x6f,  0xbc,  0x24,  0xc0,  0xf7,  0xcd,  0x46,  0x04,  0x78,  0xde],
            vec![0x79,  0x6a,  0xc4,  0x67,  0x9b,  0xaf,  0x4d,  0x78,  0xbd,  0xf3,  0xd5,  0x89],
            vec![0xcf,  0x15,  0x1a,  0x12,  0xf1,  0x59,  0xa2,  0x23,  0x13,  0x9e,  0x2b,  0x34],
            vec![0x24,  0xc0,  0xf7,  0xcd,  0x46,  0x04,  0x78,  0xde,  0x68,  0x48,  0x80,  0xef],
            vec![0x9b,  0xaf,  0x4d,  0x78,  0xbd,  0xf3,  0xd5,  0x89,  0xde,  0x37,  0x5e,  0x9a],
            vec![0xf1,  0x59,  0xa2,  0x23,  0x13,  0x9e,  0x2b,  0x34,  0x35,  0xe2,  0xb3,  0x45],
            vec![0x46,  0x04,  0x78,  0xde,  0x68,  0x48,  0x80,  0xef,  0x8a,  0x8d,  0x09,  0xf0],
            vec![0xbd,  0xf3,  0xd5,  0x89,  0xde,  0x37,  0x5e,  0x9a,  0xe0,  0x7b,  0xe6,  0xab],
        ];


        let key = vec![0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89];
        let mut key_block = create_key_block(&key);
        assert!(key_block == 0xabcdef0123456789);

        let mut actual: Vec<Vec<u8>> = vec![];

        for r in 0..NUM_ROUNDS {
            let subkeys = generate_subkeys(&mut key_block, r);
            assert!(subkeys.len() == COL_LEN);
            actual.push(subkeys);
        }

        assert!(actual == expected);
    }

    #[test]
    fn test_block_creation() {
        let mut r0 = 0xaaee;
        let mut r1 = 0xaa66;
        let mut r2 = 0xaaee;
        let mut r3 = 0xaa66;
        let mut key_block = 0xabcdef0123456789;

        let (f0, f1) = f(r0, r1, &mut key_block, 0);
        assert!(f0 == 0x3eb1);
        assert!(f1 == 0xa4e9);
        let temp_r2 = r2;
        let temp_r3 = r3;
        r2 = r0;
        r3 = r1;
        r0 = temp_r2 ^ f0 as u16;
        r1 = temp_r3 ^ f1 as u16;

        let block = to_u32_vec(&vec![r0, r1, r2, r3]);
        let block = ((block[0] as u64) << 32) | block[1] as u64;

        assert!(block == 0x945f0e8faaeeaa66);
    }

    #[test]
    // #[ignore]
    fn test_subkey_generation_in_f_function() {
        let expected: Vec<u64> = vec![
            0x945f0e8faaeeaa66,
            0x24c7cb70945f0e8f,
            0x99efc5d324c7cb70,
            0xe0562f3499efc5d3,
            0x708dbb8ce0562f34,
            0xb3a5bb1c708dbb8c,
            0xba991c1bb3a5bb1c,
            0x9ad4d197ba991c1b,
            0xb7538cf69ad4d197,
            0x559ecc0bb7538cf6,
            0x2df37aaf559ecc0b,
            0x721b9b4c2df37aaf,
            0x906cce55721b9b4c,
            0xccd1ac27906cce55,
            0x811429e4ccd1ac27,
            0x9bbb3172811429e4,
        ];

        let mut r0 = 0xaaee;
        let mut r1 = 0xaa66;
        let mut r2 = 0xaaee;
        let mut r3 = 0xaa66;
        let mut key_block = 0xabcdef0123456789;

        let mut blocks = vec![];

         for r in 0..NUM_ROUNDS {
            let (f0, f1) = f(r0, r1, &mut key_block, r);
            if r == 0 {
                assert!(f0 == 0x3eb1);
                assert!(f1 == 0xa4e9);
            }
            let temp_r2 = r2;
            let temp_r3 = r3;
            r2 = r0;
            r3 = r1;
            r0 = temp_r2 ^ f0 as u16;
            r1 = temp_r3 ^ f1 as u16;

            let block = to_u32_vec(&vec![r0, r1, r2, r3]);
            let block = ((block[0] as u64) << 32) | block[1] as u64;
            blocks.push(block);
        }

        // println!("Blocks:");
        // for block in &blocks {
        //     println!("{:x}", block);
        // }

        assert!(blocks == expected);
    }

    #[test]
    fn test_output_whitening() {
        let block = 0x9bbb3172811429e4;
        let key: Vec<u8> = vec![0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89];

        let r0 = 0x9bbb;
        let r1 = 0x3172;
        let r2 = 0x8114;
        let r3 = 0x29e4;

        let y = vec![r2, r3, r0, r1];

        let ciphertext = whiten_output(&create_whitening_blocks(&key), &y);

        assert!(ciphertext == 0x2ad9c6e5b8fe56fb);
    }
}

fn whiten_output(key: &Vec<u16>, y: &Vec<u16>) -> u64 {
    let ciphertext = whiten_blocks(y, key);
    let ciphertext = to_u32_vec(&ciphertext);

    ((ciphertext[0] as u64) << 32) | ciphertext[1] as u64
}

fn whiten_input(key: &Vec<u8>, plaintext: &Vec<u8>) -> Vec<u16> {
    let key_blocks = create_whitening_blocks(key);
    let plaintext_blocks = create_whitening_blocks(plaintext);

    whiten_blocks(&key_blocks, &plaintext_blocks)
}

fn k(x: usize, key: &u64) -> u8 {
    let key = key.swap_bytes();
    let mut bits = vec![];

    // Well, this is jank
    let mut i = 56;
    while i >= 0 {
        bits.push((key >> i) as u8);
        i -= 8;
    }

    bits[x % 8]
}

fn generate_subkeys(key_block: &mut u64, r: usize) -> Vec<u8> {
    let mut subkeys = vec![];
    for _ in 0..SUBKEY_GEN_ROUNDS {
        for i in 0..SUBKEY_GEN_ROUNDS+1 {
            *key_block = key_block.rotate_left(1);
            let subkey = k(4*r + i, &key_block);
            subkeys.push(subkey);
        }
    }

    subkeys
}

#[allow(dead_code)]
fn get_f_table_value(index: u8) -> u8 {
    let row = (index >> 4) as usize;
    // I hope there's a more elegant way to find a column
    let col = (index.rotate_right(4) >> 4) as usize;

    F_TABLE[row*ROW_LEN + col]
}

#[allow(dead_code)]
fn g(r: u16, subkeys: &[u8], _round: usize) -> u16 {
    let g1 = (r >> 8) as u8;
    let g2 = r as u8;
    
    let g3 = get_f_table_value(g2 ^ subkeys[0]) ^ g1;
    let g4 = get_f_table_value(g3 ^ subkeys[1]) ^ g2;
    let g5 = get_f_table_value(g4 ^ subkeys[2]) ^ g3;
    let g6 = get_f_table_value(g5 ^ subkeys[3]) ^ g4;

    to_u16_block(&g5, &g6)
}

#[allow(dead_code)]
fn f(r0: u16, r1: u16, key_block: &mut u64, round: usize) -> (u16, u16) {
    let subkeys = generate_subkeys(key_block, round);

    let t0 = g(r0, &subkeys[0..4], round);
    let t1 = g(r1, &subkeys[4..8], round);

    let sum = t0 as u32 + 2*t1 as u32 + to_u16_block(&subkeys[8], &subkeys[9]) as u32; 
    let f0 = (sum % 2u32.pow(16)) as u16;

    let sum = 2*t0 as u32 + t1 as u32 + to_u16_block(&subkeys[10], &subkeys[11]) as u32;
    let f1 = (sum % 2u32.pow(16)) as u16;

    (f0, f1)
}

#[allow(dead_code)]
fn create_whitening_blocks(bytes: &Vec<u8>) -> Vec<u16> {
    let mut blocks = vec![];

    let mut iter = bytes.iter();
    while let Some(first) = iter.next() {
        let second = iter.next().unwrap();
        let answer = to_u16_block(&first, &second);
        blocks.push(answer);
    }

    blocks
}

#[allow(dead_code)]
fn whiten_blocks(key_blocks: &Vec<u16>, plaintext_blocks: &Vec<u16>) -> Vec<u16> {
    let mut results = vec![];
    for blocks in key_blocks.iter().zip(plaintext_blocks.iter()) {
        let result = blocks.0 ^ blocks.1;
        results.push(result);
    }

    results
}

fn to_u16_block(first: &u8, second: &u8) -> u16 {
    let first = (*first as u16) << 8;
    let second = *second as u16;

    first | second
}

fn to_u32_block(first: &u16, second: &u16) -> u32 {
    let first = (*first as u32) << 16;
    let second = *second as u32;

    first | second
}

fn to_u64_block(first: &u32, second: &u32) -> u64 {
    let first = (*first as u64) << 16;
    let second = *second as u64;

    first | second
}

#[allow(dead_code)]
fn to_u16_vec(key: &Vec<u8>) -> Vec<u16> {
    let mut iter = key.iter();
    let mut blocks: Vec<u16> = vec![];
    while let Some(first) = iter.next() {
        let second = iter.next().unwrap();
        let block = to_u16_block(&first, &second);
        blocks.push(block);
    }

    blocks
}

#[allow(dead_code)]
fn to_u32_vec(key: &Vec<u16>) -> Vec<u32> {
    let mut iter = key.iter();
    let mut blocks: Vec<u32> = vec![];
    while let Some(first) = iter.next() {
        let second = iter.next().unwrap();
        let block = to_u32_block(&first, &second);
        blocks.push(block);
    }

    blocks
}

#[allow(dead_code)]
fn create_key_block(key: &Vec<u8>) -> u64 {
    let key = to_u16_vec(&key);
    let key = to_u32_vec(&key);
    
    ((key[0] as u64) << 32) | key[1] as u64
}

// fn encrypt(key: &Vec<u16>, plaintext: &Vec<u16>) -> u64 {
    // let key_blocks = create_whitening_blocks(key);
    // let plaintext_blocks = create_whitening_blocks(plaintext);

//     let results = whiten_blocks(key_blocks, plaintext_blocks);

//     assert!(results == [0xaaee, 0xaa66, 0xaaee, 0xaa66]);

//     let mut r0 = results[0];
//     let mut r1 = results[1];
//     let mut r2 = results[2];
//     let mut r3 = results[3];

    // for r in 0..NUM_ROUNDS {
    //     let (f0, f1) = f(r0, r1, r);
    //     let temp_r2 = r2;
    //     let temp_r3 = r3;
    //     r2 = r0;
    //     r3 = r1;
    //     r0 = temp_r2 ^ f0 as u16;
    //     r1 = temp_r3 ^ f1 as u16;
    // }

//     ciphertext
// }

fn main() {
    // let key: u64 = 0xabcdef0123456789;
    // let key: Vec<u8> = vec![0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89];
}
