use sha2::{Digest, Sha256, Sha512};

/// A derived hash result returned by the hashing helpers.
///
/// - `string` contains the hexadecimal representation of the final hash (lowercase).
/// - `byte_slice` contains the ASCII bytes of that hexadecimal string (i.e. string.as_bytes().to_vec()).
///
/// Both fields are provided for convenience depending on whether callers want the textual
/// hex representation or the raw byte slice of that text.
#[derive(Debug, Clone)]
pub struct DHash {
    /// The final hash encoded as ASCII hex bytes (string.as_bytes()).
    pub byte_slice: Vec<u8>,
    /// The final hash as a hex string.
    pub string: String,
}

/// Compute a modified SHA-256 based hash.
///
/// This function first computes the SHA-256 digest of `str_data` and encodes it as a
/// lowercase hexadecimal string. Then, for `repeat` iterations, it removes every
/// `delete_step`-th character (1-based indexing) from the current hex string and
/// computes the SHA-256 digest of the resulting string, updating the hex string to
/// the new digest each iteration.
///
/// Parameters
/// - `str_data`: input string to hash
/// - `delete_step`: positive step value indicating which characters to drop (must be > 0)
/// - `repeat`: number of times to perform the drop-and-hash cycle after the initial hash
///
/// Panics
/// - If `delete_step` is 0.
///
/// Returns
/// - `DHash` containing the final hex string and its ASCII byte slice.
pub fn d256(str_data: &str, delete_step: usize, repeat: usize) -> DHash {

    assert!(delete_step > 0, "delete_step must be > 0");

    let mut hash_bytes = Sha256::digest(str_data.as_bytes());
    let mut str_data_hash = hex::encode(&hash_bytes);

    for _ in 0..repeat {
        let mut result = String::with_capacity(str_data_hash.len());
        for (i, ch) in str_data_hash.chars().enumerate() {
            if (i + 1) % delete_step != 0 {
                result.push(ch);
            }
        }

        hash_bytes = Sha256::digest(result.as_bytes());
        str_data_hash = hex::encode(&hash_bytes);
    }

    DHash {
        byte_slice: str_data_hash.clone().into_bytes(),
        string: str_data_hash,
    }
    
}

/// Compute a modified SHA-512 based hash.
///
/// Behavior mirrors `d256` but uses SHA-512 for digesting.
/// First computes SHA-512 of `str_data` and hex-encodes it, then performs `repeat`
/// iterations of deleting every `delete_step`-th character from the current hex
/// string (1-based) and re-hashing the result with SHA-512.
///
/// Parameters
/// - `str_data`: input string to hash
/// - `delete_step`: positive step value indicating which characters to drop (must be > 0)
/// - `repeat`: number of times to perform the drop-and-hash cycle after the initial hash
///
/// Panics
/// - If `delete_step` is 0.
///
/// Returns
/// - `DHash` containing the final hex string and its ASCII byte slice.
pub fn d512(str_data: &str, delete_step: usize, repeat: usize) -> DHash {

    assert!(delete_step > 0, "delete_step must be > 0");

    let mut hash_bytes = Sha512::digest(str_data.as_bytes());
    let mut str_data_hash = hex::encode(&hash_bytes);

    for _ in 0..repeat {
        let mut result = String::with_capacity(str_data_hash.len());
        for (i, ch) in str_data_hash.chars().enumerate() {
            if (i + 1) % delete_step != 0 {
                result.push(ch);
            }
        }

        hash_bytes = Sha512::digest(result.as_bytes());
        str_data_hash = hex::encode(&hash_bytes);
    }

    DHash {
        byte_slice: str_data_hash.clone().into_bytes(),
        string: str_data_hash,
    }

}

/// Compute a composite SHA-256 based hash following a path specification.
///
/// The function first computes the SHA-256 digest of `str_data` and hex-encodes it.
/// Then it iterates over the path segments separated by '/'. Each segment may contain
/// an optional suffix separated by '#' where the part before '#' is an "addon string"
/// and the part after '#' is an integer `delete_step`.
///
/// For each segment:
/// - Parse the segment as `addon#step` (the `#step` portion is required and must parse as usize).
/// - Concatenate the current hash hex string with `addon`.
/// - Call `d256` on that concatenation with `delete_step` and `repeat = 1`.
/// - Use the returned hash string as the current hash for the next segment.
///
/// Example path: "-A#3/-B#2" will first append "-A" and run d256(..., 3, 1), then append
/// "-B" to the resulting hash and run d256(..., 2, 1).
///
/// Panics
/// - If any segment's `step` part is missing or cannot be parsed as a usize, the function will panic.
///
/// Returns
/// - `DHash` containing the final hex string and its ASCII byte slice.
pub fn d256c(str_data: &str, path: &str) -> DHash {

    let mut hasher = Sha256::new();
    hasher.update(str_data.as_bytes());
    let result = hasher.finalize();
    let mut str_data_hash = hex::encode(result);

    for part in path.split('/') {
        let mut pieces = part.splitn(2, '#');
        let addon_string = pieces.next().unwrap_or("");
        let step_str = pieces.next().unwrap_or("");

        let delete_step: usize = match step_str.parse() {
            Ok(n) => n,
            Err(_) => panic!("Hashing path is incorrect!"),
        };

        let new_string = format!("{}{}", str_data_hash, addon_string);

        str_data_hash = d256(&new_string, delete_step, 1).string;
    }

    DHash {
        byte_slice: str_data_hash.clone().into_bytes(),
        string: str_data_hash,
    }

}

/// Compute a composite SHA-512 based hash following a path specification.
///
/// Behavior mirrors `d256c` but uses SHA-512 as the underlying hash function.
///
/// Panics
/// - If any segment's `step` part is missing or cannot be parsed as a usize, the function will panic.
///
/// Returns
/// - `DHash` containing the final hex string and its ASCII byte slice.
pub fn d512c(str_data: &str, path: &str) -> DHash {

    let mut hasher = Sha512::new();
    hasher.update(str_data.as_bytes());
    let result = hasher.finalize();
    let mut str_data_hash = hex::encode(result);

    for part in path.split('/') {
        let mut pieces = part.splitn(2, '#');
        let addon_string = pieces.next().unwrap_or("");
        let step_str = pieces.next().unwrap_or("");

        let delete_step: usize = match step_str.parse() {
            Ok(n) => n,
            Err(_) => panic!("Hashing path is incorrect!"),
        };

        let new_string = format!("{}{}", str_data_hash, addon_string);

        str_data_hash = d512(&new_string, delete_step, 1).string;
    }

    DHash {
        byte_slice: str_data_hash.clone().into_bytes(),
        string: str_data_hash,
    }

}
