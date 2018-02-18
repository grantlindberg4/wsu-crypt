use super::*;

pub fn create_whitening_blocks(bytes: &Vec<u8>) -> Vec<u16> {
    let mut blocks = vec![];

    let mut iter = bytes.iter();
    while let Some(first) = iter.next() {
        let second = iter.next().unwrap();
        let answer = convert_types::to_u16_block(&first, &second);
        blocks.push(answer);
    }

    blocks
}

pub fn whiten_blocks(key_blocks: &Vec<u16>, plaintext_blocks: &Vec<u16>) -> Vec<u16> {
    let mut results = vec![];
    for blocks in key_blocks.iter().zip(plaintext_blocks.iter()) {
        let result = blocks.0 ^ blocks.1;
        results.push(result);
    }

    results
}

pub fn whiten_output(key: &Vec<u16>, y: &Vec<u16>) -> u64 {
    let ciphertext = whiten::whiten_blocks(y, key);
    let ciphertext = convert_types::to_u32_vec(&ciphertext);

    ((ciphertext[0] as u64) << 32) | ciphertext[1] as u64
}

pub fn whiten_input(key: &Vec<u8>, plaintext: &Vec<u8>) -> Vec<u16> {
    let key_blocks = whiten::create_whitening_blocks(key);
    let plaintext_blocks = whiten::create_whitening_blocks(plaintext);

    whiten::whiten_blocks(&key_blocks, &plaintext_blocks)
}
