use rand::Rng;
use std::iter;

/// Id of commits
/// The size of bytes must be 20!
#[derive(Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct Id {
    bytes: Box<[u8]>,
}

impl Id {
    pub fn new() -> Self {
        let mut bytes = Vec::with_capacity(20);
        let mut rng = rand::thread_rng();

        bytes.extend(iter::repeat(rng.gen::<u8>()).take(20));

        println!("{:?}", bytes);

        Id {
            bytes: bytes.into_boxed_slice(),
        }
    }
}
