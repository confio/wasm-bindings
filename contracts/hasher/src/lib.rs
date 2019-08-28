use sha2::{Digest, Sha256};

#[no_mangle]
pub extern "C" fn hash(start: i32, step: i32) -> i32 {
    hash_num(start, step)
}

// hash_num creates a 32 byte array [start, start+step, start+2*step, ...]
// it then does a sha256 hash and collapses the return value to one i32 by
// adding all bytes together.
//
// weird algorithm for simple wasm interface
pub fn hash_num(start: i32, step: i32) -> i32 {
    let data = build_array(start, step);
    let hash = Sha256::digest(&data);
    let mut sum = 0;
    for i in 0..16 {
        sum += hash[i] as i32;
    }
    return sum
}

fn build_array(start: i32, step: i32) -> [u8; 32] {
    let mut res = [0u8; 32];
    let mut val = start;
    for i in 0..32 {
        res[i] = val  as u8;
        val = (val + step) % 256;
    }
    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_array() {
        let res = build_array(7, 19);
        // python: [(7 + 19*x % 256) for x in range(32)]
        let expected = [7, 26, 45, 64, 83, 102, 121, 140, 159, 178, 197, 216, 235, 254, 17, 36, 55, 74, 93, 112, 131, 150, 169, 188, 207, 226, 245, 8, 27, 46, 65, 84];
        assert_eq!(&expected, &res);
    }

    #[test]
    fn test_hash_num() {
        let res = hash_num(19, 20);
        let expected = 2300;
        assert_eq!(expected, res);
    }
}
