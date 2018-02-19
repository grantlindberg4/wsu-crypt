use super::*;

/// # Description
/// Takes a vector of bytes and concatenates them to create a vector of unsigned 16-bit integers (blocks) to be used for whitening
///
/// # Arguments
/// * `bytes` - A vector of bytes representing the key, plaintext, and/or ciphertext depending on whether en/decryption is being used
///
/// # Example
/// let key: Vec<u8> = vec![0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89];
/// 
/// let blocks = whiten::create_whitening_blocks(&key);
/// assert!(blocks == [0xabcd, 0xef01, 0x2345, 0x6789]);
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

/// # Description
/// Takes a vector of bytes and concatenates them to create a vector of unsigned 16-bit integers (blocks) to be used for whitening
///
/// # Arguments
/// * `key_blocks` - A vector of unsigned 16-bit integers representing the key blocks. Retrieved from whiten::create_whitening_blocks() result
/// * `plaintext_blocks` - A vector of unsigned 16-bit integers representing the plaintext blocks. Retrieved from whiten::create_whitening_blocks() result
///
/// # Example
/// let plaintext_blocks = vec![0x0123, 0x4567, 0x89ab, 0xcdef];
/// let key_blocks = vec![0xabcd, 0xef01, 0x2345, 0x6789];
/// 
/// let blocks = whiten::whiten_blocks(&key_blocks, &plaintext_blocks);
/// assert!(blocks == [0xaaee, 0xaa66, 0xaaee, 0xaa66]);
pub fn whiten_blocks(key_blocks: &Vec<u16>, plaintext_blocks: &Vec<u16>) -> Vec<u16> {
    let mut results = vec![];
    for blocks in key_blocks.iter().zip(plaintext_blocks.iter()) {
        let result = blocks.0 ^ blocks.1;
        results.push(result);
    }

    results
}

/// # Description
/// A wrapper over the block creation stage and the block whitening stage for output
/// Calls whiten::whiten_blocks() on the key and converts the result to an unsigned 32-bit vector, which should only contain two elements
/// Concatenates the two 32-bit elements and returns an unsigned 64-bit block
///
/// # Arguments
/// * `key` - A vector of unsigned 16-bit integers representing the key blocks. The block creation stage must be performed before it is passed into this function.
/// * `y` - A vector of unsigned 16-bit integers representing the r values by the end of the en/decryption process. Used in the whitening stage to obtain the final result
///
/// # Example
/// let key: Vec<u8> = vec![0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89];
/// 
/// let r0 = 0x9bbb;
/// let r1 = 0x3172;
/// let r2 = 0x8114;
/// let r3 = 0x29e4;
/// 
/// let y = vec![r2, r3, r0, r1];
/// 
/// let ciphertext = whiten::whiten_output(&whiten::create_whitening_blocks(&key), &y);
/// 
/// assert!(ciphertext == 0x2ad9c6e5b8fe56fb);
pub fn whiten_output(key: &Vec<u16>, y: &Vec<u16>) -> u64 {
    let result = whiten::whiten_blocks(y, key);
    let result = convert_types::to_u32_vec(&result);

    ((result[0] as u64) << 32) | result[1] as u64
}

/// # Description
/// A wrapper over the block creation stage and the block whitening stage for input
/// Calls whiten::create_whitening_blocks() on both the key and plaintext, processes them with whitening, and returns the result
/// See whiten::create_whitening_blocks() and whiten::whiten_blocks() for examples
///
/// # Arguments
/// * `key_blocks` - A vector of bytes representing the key blocks. Note that the block creation stage takes place within this function.
/// * `plaintext_blocks` - A vector of bytes representing the plaintext blocks. Note that the block creation stage takes place within this function.
///
/// # Example
/// let key: Vec<u8> = vec![0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89];
/// let key_blocks = whiten::create_whitening_blocks(&key)
/// let plaintext: Vec<u8> = vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
/// let plaintext_blocks = whiten::create_whitening_blocks(&key)
/// 
/// let blocks = whiten::whiten_blocks(&key_blocks, &plaintext_blocks);
/// assert!(blocks == [0xaaee, 0xaa66, 0xaaee, 0xaa66]);
pub fn whiten_input(key: &Vec<u8>, plaintext: &Vec<u8>) -> Vec<u16> {
    let key_blocks = whiten::create_whitening_blocks(key);
    let plaintext_blocks = whiten::create_whitening_blocks(plaintext);

    whiten::whiten_blocks(&key_blocks, &plaintext_blocks)
}
