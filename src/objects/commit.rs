use crate::hash::Hash;
use crate::objects::blob::Blob;
use crate::user::User;
use crate::ApplicationError;
use chrono::DateTime;
use chrono::Utc;

#[derive(Debug)]
pub struct Commit {
    id: Hash,
    user: User,
    date: DateTime<Utc>, // we should use to_rfc2822/parse_from_rfc2822
    blob: Blob,
}

impl Commit {
    pub fn new(_raw: &str) -> Result<Commit, ApplicationError> {
        unimplemented!()
    }

    pub fn serialize(&self) -> Result<String, ApplicationError> {
        unimplemented!()
    }
}
