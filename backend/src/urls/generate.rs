use std::string::FromUtf8Error;

const ALPHABET: &[u8; 62] =
    b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// FNV-1a 64-bit
fn fnv1a_64(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// use len <= 10
pub fn short_code(url: &str, len: usize) 
-> Result<String, FromUtf8Error> {
    let mut n = fnv1a_64(url.trim().as_bytes());
    let mut out = vec![0u8; len];
    for slot in out.iter_mut().rev() {
        *slot = ALPHABET[(n % 62) as usize];
        n /= 62;
    }
    String::from_utf8(out)
}

/// handles collisions by re-hashing with a salt until the code is available
pub fn unique_code(url: &str, len: usize, exists: impl Fn(&str) -> bool) 
-> Result<String, FromUtf8Error> {
    let mut attempt = 0u32;
    loop {
        let salted = if attempt == 0 {
            url.to_string()
        } else {
            format!("{url}#{attempt}")
        };
        let code = short_code(&salted, len)?;
        if !exists(&code) {
            return Ok(code);
        }
        attempt += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_url_gives_same_code() {
        let a = short_code("https://example.com", 7);
        let b = short_code("https://example.com", 7);
        assert_eq!(a, b);
    }

    #[test]
    fn different_urls_give_different_codes() {
        let a = short_code("https://example.com/a", 7);
        let b = short_code("https://example.com/b", 7);
        assert_ne!(a, b);
    }

    #[test]
    fn code_has_requested_length_and_valid_chars() {
        let code = short_code("https://example.com", 7).unwrap();
        assert_eq!(code.len(), 7);
        assert!(code.bytes().all(|b| ALPHABET.contains(&b)));
    }

    #[test]
    fn surrounding_whitespace_is_ignored() {
        assert_eq!(
            short_code("  https://example.com  ", 7),
            short_code("https://example.com", 7)
        );
    }

    #[test]
    fn fnv_known_vector() {
        // Published FNV-1a test vector for the empty string
        assert_eq!(fnv1a_64(b""), 0xcbf29ce484222325);
    }
}