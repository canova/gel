use hex;
use log::info;

/// Id of commits
/// The size of bytes must be 20!
#[derive(Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct Id {
    bytes: [u8; 20],
}

impl Id {
    pub fn new() -> Self {
        info!("Creating a new Id");
        let mut sha1 = sha1::Sha1::new();
        sha1.update(b"test!");

        let bytes = sha1.digest().bytes();
        Id { bytes }
    }

    pub fn to_string(&self) -> String {
        hex::encode(self.bytes)
    }
}
