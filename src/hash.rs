use hex;
use log::info;

/// Hash of commits
/// The size of bytes must be 20!
#[derive(Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct Hash {
    bytes: Box<[u8]>,
}

impl Hash {
    pub fn new(content: &str) -> Self {
        info!("Creating a new Id");
        let mut sha1 = sha1::Sha1::new();
        sha1.update(content.as_bytes());

        let bytes = Box::new(sha1.digest().bytes());
        Hash { bytes }
    }

    pub fn from_hex(hash: &str) -> Self {
        let hex = hex::decode(hash).unwrap().into_boxed_slice();
        Hash { bytes: hex }
    }

    pub fn to_string(&self) -> String {
        hex::encode(&self.bytes)
    }

    // TODO: explain why we need this
    pub fn to_string_parts(&self) -> (String, String) {
        let s = hex::encode(&self.bytes);
        (s[..2].to_string(), s[2..].to_string())
    }
}
