const SUBKEY_GEN_ROUNDS: usize = 3;

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

pub fn generate_subkeys_for_decrypt(key_block: &mut u64, r: usize) -> Vec<u8> {
    let mut subkeys = vec![];
    for _ in 0..SUBKEY_GEN_ROUNDS {
        for i in (0..SUBKEY_GEN_ROUNDS+1).rev() {
            let subkey = k(4*r + i, &key_block);
            subkeys.push(subkey);
            *key_block = key_block.rotate_right(1);
        }
    }

    subkeys
}
