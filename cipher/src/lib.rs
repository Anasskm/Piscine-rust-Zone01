#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String,
}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError {
            validation,
            expected,
        }
    }
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if original.is_empty() || ciphered.is_empty() {
        return None;
    }

    let expected = atbash_cipher(original);

    if expected == ciphered {
        Some(Ok(true))
    } else {
        Some(Err(CipherError::new(false, expected)))
    }
}

fn atbash_cipher(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let b = if c.is_ascii_lowercase() { b'z' } else { b'Z' };
                let mirrored = (base + (b - c as u8)) as char;
                mirrored
            } else {
                c
            }
        })
        .collect()
}