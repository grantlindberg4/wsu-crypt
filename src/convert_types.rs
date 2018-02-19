/// # Description
/// Takes two unsigned 8-bit integers and concatenates them into an unsigned 16-bit integer
///
/// # Arguments
/// * `first` - an unsigned 8-bit integer
/// * `second` - an unsigned 8-bit integer
///
/// # Example
/// let first = 0xab;
/// let second = 0xcd;
/// let together = convert_types::to_16_block(&first, &second);
/// assert!(together == 0xabcd);
pub fn to_u16_block(first: &u8, second: &u8) -> u16 {
    let first = (*first as u16) << 8;
    let second = *second as u16;

    first | second
}

/// # Description
/// Takes two unsigned 16-bit integers and concatenates them into an unsigned 32-bit integer
///
/// # Arguments
/// * `first` - an unsigned 16-bit integer
/// * `second` - an unsigned 16-bit integer
///
/// # Example
/// let first = 0xabcd;
/// let second = 0x0123;
/// let together = convert_types::to_32_block(&first, &second);
/// assert!(together == 0xabcd0123);
pub fn to_u32_block(first: &u16, second: &u16) -> u32 {
    let first = (*first as u32) << 16;
    let second = *second as u32;

    first | second
}

/// # Description
/// Takes a vector of unsigned 8-bit integers and transforms it into a vector of unsigned 16-bit integers
///
/// # Arguments
/// * `key` - a vector of unsigned 8-bit integers
///
/// # Example
/// let key = vec![0xab, 0xcd, 0xef, 0x01];
/// let key = convert_types::to_16_vec(&key);
/// assert!(key == vec![0xabcd, 0xef01]);
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

/// Takes a vector of unsigned 16-bit integers and transforms it into a vector of unsigned 32-bit integers
/// # Arguments
/// * `key` - a vector of unsigned 16-bit integers
/// # Example
/// let key == vec![0xabcd, 0xef01];
/// let key = convert_types::to_32_vec(&key);
/// assert!(key == vec![0xabcdef01]);
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

/// Takes a vector of unsigned 8-bit integers and creates an unsigned 64-bit integer representation of that vector
/// # Arguments
/// * `key` - a vector of unsigned 8-bit integers
/// # Example
/// let key = vec![0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89];
/// let key = convert_types::create_key_block(&key);
/// assert!(key == 0xabcdef0123456789);
pub fn create_key_block(key: &Vec<u8>) -> u64 {
    let key = to_u16_vec(&key);
    let key = to_u32_vec(&key);
    
    ((key[0] as u64) << 32) | key[1] as u64
}
