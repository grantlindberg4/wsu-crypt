const SUBKEY_GEN_ROUNDS: usize = 3;

/// # Description
/// Takes an unsigned 64-bit key block, performs operations on it, and uses an integer to pick out 8 bits to be used for subkey generation
///
/// # Arguments
/// * `x` - A pointer-sized integer. This value is determined by the round number and key number and used to pick out 8 bits to be returned.
/// * `key` - An unsigned 64-bit integer. Its bytes are swapped, and it is broken up into a vector of bytes in order to pick out the specific bits.
///
/// # Example
/// let mut key_block: u64 = 0xabcdef0123456789;
/// key_block = key_block.rotate_left(1);
/// let subkey = subkey_gen::k(4*0 + 0, &key_block);
/// 
/// assert!(subkey == 0x13);
pub fn k(x: usize, key: &u64) -> u8 {
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

/// # Description
/// Generates a vector of 12 subkeys to be used in the F function for encryption
/// Note that for each round, the key block is first rotated left and then the key bits are selected
///
/// # Arguments
/// * `key_block` - An unsigned 64-bit integer. Used as a `key` to determine the subkeys.
/// * `r` - A pointer-sized integer. This value is determined by the round number and key number and used to pick out 8 bits to be returned.
///
/// # Example
/// let mut key_block = 0xabcdef0123456789;
/// let expected = vec![0x13,  0x9e,  0x2b,  0x34,  0x35,  0xe2,  0xb3,  0x45,  0x57,  0x26,  0x3c,  0x56];
/// let subkeys = subkey_gen::generate_subkeys_for_encrypt(&mut key_block, 0);
/// assert!(subkeys == expected);
pub fn generate_subkeys_for_encrypt(key_block: &mut u64, r: usize) -> Vec<u8> {
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

/// # Description
/// See documentation for subkey_gen::generate_subkeys_for_encrypt()
/// Performs similar operations but generates the keys in the reverse order
/// Unlike the process used for encryption, the bits are selected first, and then the key is rotated right
/// Note also that the subkeys are returned in the reverse order
///
/// # Example
/// let mut key_block = 0xabcdef0123456789;
/// let expected = vec![0xbd,  0xf3,  0xd5,  0x89,  0xde,  0x37,  0x5e,  0x9a,  0xe0,  0x7b,  0xe6,  0xab];
/// let subkeys = subkey_gen::generate_subkeys_for_decrypt(&mut key_block, 0);
/// assert!(subkeys == expected);
pub fn generate_subkeys_for_decrypt(key_block: &mut u64, r: usize) -> Vec<u8> {
    let mut subkeys = vec![];
    for _ in 0..SUBKEY_GEN_ROUNDS {
        for i in (0..SUBKEY_GEN_ROUNDS+1).rev() {
            let subkey = k(4*r + i, &key_block);
            subkeys.push(subkey);
            *key_block = key_block.rotate_right(1);
        }
    }

    subkeys.reverse();
    subkeys
}
