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
const NUM_ROUNDS: usize = 16;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_whitening_stage() {
        let key = vec![0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89];
        let plaintext = vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];

        let key_blocks = create_blocks(key);
        let plaintext_blocks = create_blocks(plaintext);

        let results = whiten_blocks(key_blocks, plaintext_blocks);
        assert!(results == [0xaaee, 0xaa66, 0xaaee, 0xaa66]);
    }

    #[test]
    #[ignore]
    fn test_g_function() {
        let r0 = 0xaaee;
        let r1 = 0xaa66;
        let subkeys = [
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
    #[ignore]
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
    #[ignore]
    fn test_f_function() {
        let r0 = 0xaaee;
        let r1 = 0xaa66;
        let subkeys = [
            0x13, 0x9e, 0x2b, 0x34, 0x35, 0xe2,
            0xb3, 0x45, 0x57, 0x26, 0x3c, 0x56
        ];

        let t0 = g(r0, &subkeys[0..4], 0);
        assert!(t0 == 0xf889);
        let t1 = g(r1, &subkeys[4..8], 0);
        assert!(t1 == 0x7781);
        let f0 = (t0 as u32 + 2*t1 as u32 + concat(subkeys[8], subkeys[9]) as u32) % 2u32.pow(16);
        assert!(f0 == 0x3eb1);
        let f1 = (2*t0 as u32 + t1 as u32 + concat(subkeys[10], subkeys[11]) as u32) % 2u32.pow(16);
        assert!(f1 == 0xa4e9);
    }

    #[test]
    fn test_subkey_generation() {
        // let expected: [u16; 192] = [
        //     0x13,  0x9e,  0x2b,  0x34,  0x35,  0xe2,  0xb3,  0x45,  0x57,  0x26,  0x3c,  0x56,
        //     0x68,  0x48,  0x80,  0xef,  0x8a,  0x8d,  0x09,  0xf0,  0xac,  0xd1,  0x91,  0x01,
        //     0xde,  0x37,  0x5e,  0x9a,  0xe0,  0x7b,  0xe6,  0xab,  0x02,  0xbc,  0x6f,  0xbc,
        //     0x35,  0xe2,  0xb3,  0x45,  0x57,  0x26,  0x3c,  0x56,  0x79,  0x6a,  0xc4,  0x67,
        //     0x8a,  0x8d,  0x09,  0xf0,  0xac,  0xd1,  0x91,  0x01,  0xcf,  0x15,  0x1a,  0x12,
        //     0xe0,  0x7b,  0xe6,  0xab,  0x02,  0xbc,  0x6f,  0xbc,  0x24,  0xc0,  0xf7,  0xcd,
        //     0x57,  0x26,  0x3c,  0x56,  0x79,  0x6a,  0xc4,  0x67,  0x9b,  0xaf,  0x4d,  0x78,
        //     0xac,  0xd1,  0x91,  0x01,  0xcf,  0x15,  0x1a,  0x12,  0xf1,  0x59,  0xa2,  0x23,
        //     0x02,  0xbc,  0x6f,  0xbc,  0x24,  0xc0,  0xf7,  0xcd,  0x46,  0x04,  0x78,  0xde,
        //     0x79,  0x6a,  0xc4,  0x67,  0x9b,  0xaf,  0x4d,  0x78,  0xbd,  0xf3,  0xd5,  0x89,
        //     0xcf,  0x15,  0x1a,  0x12,  0xf1,  0x59,  0xa2,  0x23,  0x13,  0x9e,  0x2b,  0x34,
        //     0x24,  0xc0,  0xf7,  0xcd,  0x46,  0x04,  0x78,  0xde,  0x68,  0x48,  0x80,  0xef,
        //     0x9b,  0xaf,  0x4d,  0x78,  0xbd,  0xf3,  0xd5,  0x89,  0xde,  0x37,  0x5e,  0x9a,
        //     0xf1,  0x59,  0xa2,  0x23,  0x13,  0x9e,  0x2b,  0x34,  0x35,  0xe2,  0xb3,  0x45,
        //     0x46,  0x04,  0x78,  0xde,  0x68,  0x48,  0x80,  0xef,  0x8a,  0x8d,  0x09,  0xf0,
        //     0xbd,  0xf3,  0xd5,  0x89,  0xde,  0x37,  0x5e,  0x9a,  0xe0,  0x7b,  0xe6,  0xab,
        // ];

        let expected = vec![0x13,  0x9e,  0x2b,  0x34,  0x35,  0xe2,  0xb3,  0x45,  0x57,  0x26,  0x3c,  0x56];
        let mut subkeys = vec![];

        // let key = vec![0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89];
        let key: Vec<u8> = vec![0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9];
        let converted = create_u64_key_from_u16(&key);

        let mut shifted = converted;
        for r in 0..1 {
            shifted = shifted.rotate_left(1);
            let k1 = k(4*r + 0, &shifted);
            subkeys.push(k1);
            shifted = shifted.rotate_left(1);
            let k2 = k(4*r + 1, &shifted);
            subkeys.push(k2);
            shifted = shifted.rotate_left(1);
            let k3 = k(4*r + 2, &shifted);
            subkeys.push(k3);
            shifted = shifted.rotate_left(1);
            let k4 = k(4*r + 3, &shifted);
            subkeys.push(k4);

            shifted = shifted.rotate_left(1);
            let k5 = k(4*r + 0, &shifted);
            subkeys.push(k5);
            shifted = shifted.rotate_left(1);
            let k6 = k(4*r + 1, &shifted);
            subkeys.push(k6);
            shifted = shifted.rotate_left(1);
            let k7 = k(4*r + 2, &shifted);
            subkeys.push(k7);
            shifted = shifted.rotate_left(1);
            let k8 = k(4*r + 3, &shifted);
            subkeys.push(k8);

            shifted = shifted.rotate_left(1);
            let k9 = k(4*r + 0, &shifted);
            subkeys.push(k9);
            shifted = shifted.rotate_left(1);
            let k10 = k(4*r + 1, &shifted);
            subkeys.push(k10);
            shifted = shifted.rotate_left(1);
            let k11 = k(4*r + 2, &shifted);
            subkeys.push(k11);
            shifted = shifted.rotate_left(1);
            let k12 = k(4*r + 3, &shifted);
            subkeys.push(k12);
        }

        assert!(subkeys == expected);
    }
}

#[allow(dead_code)]
fn get_f_table_value(index: u8) -> u8 {
    let row = (index >> 4) as usize;
    // I hope there's a more elegant way to find a column
    let col = (index.rotate_right(4) >> 4) as usize;

    F_TABLE[row*ROW_LEN + col]
}

fn concat(first: u8, second: u8) -> u16 {
    ((first as u16) << 8) | second as u16
}

#[allow(dead_code)]
fn g(r: u16, subkeys: &[u8], round: usize) -> u16 {
    let g1 = (r >> 8) as u8;
    let g2 = r as u8;
    
    let g3 = get_f_table_value(g2 ^ subkeys[0]) ^ g1;
    let g4 = get_f_table_value(g3 ^ subkeys[1]) ^ g2;
    let g5 = get_f_table_value(g4 ^ subkeys[2]) ^ g3;
    let g6 = get_f_table_value(g5 ^ subkeys[3]) ^ g4;

    concat(g5, g6)
}

#[allow(dead_code)]
fn f(r0: u16, r1: u16, round: usize) -> (u32, u32) {
    // How do we obtain the subkeys?
    let subkeys = [
        0x13, 0x9e, 0x2b, 0x34, 0x35, 0xe2,
        0xb3, 0x45, 0x57, 0x26, 0x3c, 0x56
    ];

    let t0 = g(r0, &subkeys[0..4], round);
    let t1 = g(r1, &subkeys[4..8], round);

    let f0 = (t0 as u32 + 2*t1 as u32 + concat(subkeys[8], subkeys[9]) as u32) % 2u32.pow(16);
    let f1 = (2*t0 as u32 + t1 as u32 + concat(subkeys[10], subkeys[11]) as u32) % 2u32.pow(16);

    (f0, f1)
}

#[allow(dead_code)]
fn create_blocks(bytes: Vec<u16>) -> Vec<u16> {
    let mut blocks = vec![];

    let mut iter = bytes.iter();
    while let Some(first) = iter.next() {
        let second = iter.next().unwrap();
        let shifted = first << 8;
        let answer = shifted | second;
        blocks.push(answer);
    }

    blocks
}

#[allow(dead_code)]
fn whiten_blocks(key_blocks: Vec<u16>, plaintext_blocks: Vec<u16>) -> Vec<u16> {
    let mut results = vec![];
    for blocks in key_blocks.iter().zip(plaintext_blocks.iter()) {
        let result = blocks.0 ^ blocks.1;
        results.push(result);
    }

    results
}

fn k(x: usize, key: &u64) -> u16 {
    let key = key.swap_bytes();
    let mut bits = vec![];
    let mut i = 56;
    while i >= 0 {
        bits.push((key >> i) as u8);
        i -= 8;
    }
    // for i in &bits {
    //     println!("{:x}", i);
    // }
    bits[x % 8] as u16
}

fn to_u16_vec(key: &Vec<u8>) -> Vec<u16> {
    let mut iter = key.iter();
    let mut blocks: Vec<u16> = vec![];
    while let Some(first) = iter.next() {
        let second = iter.next().unwrap();
        let first = (*first as u16) << 4;
        let second = *second as u16;
        let block = first | second;
        blocks.push(block);
    }

    blocks
}

fn to_u32_vec(key: &Vec<u16>) -> Vec<u32> {
    let mut iter = key.iter();
    let mut blocks: Vec<u32> = vec![];
    while let Some(first) = iter.next() {
        let second = iter.next().unwrap();
        let first = (*first as u32) << 8;
        let second = *second as u32;
        let block = first | second;
        blocks.push(block);
    }

    blocks
}

fn to_u64_vec(key: &Vec<u32>) -> Vec<u64> {
    let mut iter = key.iter();
    let mut blocks: Vec<u64> = vec![];
    while let Some(first) = iter.next() {
        let second = iter.next().unwrap();
        let first = (*first as u64) << 16;
        let second = *second as u64;
        let block = first | second;
        blocks.push(block);
    }

    blocks
}

fn create_u64_key_from_u16(key: &Vec<u8>) -> u64 {
    let key = to_u16_vec(&key);
    
    let key = to_u32_vec(&key);
    let key = to_u64_vec(&key);

    (key[0] << 32) | key[1]
}

fn main() {
    // let key: u64 = 0xabcdef0123456789;
    // let key = key.rotate_left(1);
    let key: Vec<u8> = vec![0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9];
    let converted = create_u64_key_from_u16(&key);
    println!("64-bit Key: {:x}", converted);

    let mut shifted = converted;
    let mut subkeys = vec![];
    for r in 0..1 {
        shifted = shifted.rotate_left(1);
        let k1 = k(4*r + 0, &shifted);
        subkeys.push(k1);
        shifted = shifted.rotate_left(1);
        let k2 = k(4*r + 1, &shifted);
        subkeys.push(k2);
        shifted = shifted.rotate_left(1);
        let k3 = k(4*r + 2, &shifted);
        subkeys.push(k3);
        shifted = shifted.rotate_left(1);
        let k4 = k(4*r + 3, &shifted);
        subkeys.push(k4);

        shifted = shifted.rotate_left(1);
        let k5 = k(4*r + 0, &shifted);
        subkeys.push(k5);
        shifted = shifted.rotate_left(1);
        let k6 = k(4*r + 1, &shifted);
        subkeys.push(k6);
        shifted = shifted.rotate_left(1);
        let k7 = k(4*r + 2, &shifted);
        subkeys.push(k7);
        shifted = shifted.rotate_left(1);
        let k8 = k(4*r + 3, &shifted);
        subkeys.push(k8);

        shifted = shifted.rotate_left(1);
        let k9 = k(4*r + 0, &shifted);
        subkeys.push(k9);
        shifted = shifted.rotate_left(1);
        let k10 = k(4*r + 1, &shifted);
        subkeys.push(k10);
        shifted = shifted.rotate_left(1);
        let k11 = k(4*r + 2, &shifted);
        subkeys.push(k11);
        shifted = shifted.rotate_left(1);
        let k12 = k(4*r + 3, &shifted);
        subkeys.push(k12);
    }
    println!("CURRENT SUBKEYS");
    for i in &subkeys {
        println!("{:x}", i)
    }
}
