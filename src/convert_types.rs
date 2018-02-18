pub fn to_u16_block(first: &u8, second: &u8) -> u16 {
    let first = (*first as u16) << 8;
    let second = *second as u16;

    first | second
}

pub fn to_u32_block(first: &u16, second: &u16) -> u32 {
    let first = (*first as u32) << 16;
    let second = *second as u32;

    first | second
}

pub fn to_u16_vec(key: &Vec<u8>) -> Vec<u16> {
    let mut iter = key.iter();
    let mut blocks: Vec<u16> = vec![];
    while let Some(first) = iter.next() {
        let second = iter.next().unwrap();
        let block = to_u16_block(&first, &second);
        blocks.push(block);
    }

    blocks
}

pub fn to_u32_vec(key: &Vec<u16>) -> Vec<u32> {
    let mut iter = key.iter();
    let mut blocks: Vec<u32> = vec![];
    while let Some(first) = iter.next() {
        let second = iter.next().unwrap();
        let block = to_u32_block(&first, &second);
        blocks.push(block);
    }

    blocks
}

pub fn create_key_block(key: &Vec<u8>) -> u64 {
    let key = to_u16_vec(&key);
    let key = to_u32_vec(&key);
    
    ((key[0] as u64) << 32) | key[1] as u64
}
