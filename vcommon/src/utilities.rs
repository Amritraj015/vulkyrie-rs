use base64::prelude::{BASE64_STANDARD, Engine};
use sha1::{digest::FixedOutput, Digest, Sha1};

pub fn base64_encode(data: &[u8]) -> String {
    BASE64_STANDARD.encode(data)
}

pub fn generate_sha1_hash(data: &str) -> Vec<u8> {
    let mut hasher = Sha1::new();
    hasher.update(data);
    FixedOutput::finalize_fixed(hasher).to_vec()
}
