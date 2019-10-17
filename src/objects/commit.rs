use crate::hash::Hash;
use crate::user::User;
use crate::ApplicationError;
use chrono::DateTime;
use chrono::Utc;

#[derive(Debug)]
pub struct Commit {
    id: Hash,
    // Future TODO: add a committer too.
    author: User,
    // TODO: I think we should move this inside User, since commit date is attached to the author data.
    date: DateTime<Utc>, // we should use to_rfc2822/parse_from_rfc2822
    commit_message: String,
    // Every commit has a parent except the first commit.
    // Future TODO: Merge commits can have multiple parents.
    // Currently we don't have merge commit concept, but may have in the future.
    parent: Option<Hash>,
    tree: Hash,
}

impl Commit {
    pub fn new(_raw: &str) -> Result<Commit, ApplicationError> {
        unimplemented!()
    }

    pub fn serialize(&self) -> Result<String, ApplicationError> {
        unimplemented!()
    }
}
