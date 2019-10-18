mod blob;
mod commit;
mod tree;

use crate::objects::blob::Blob;
use crate::objects::tree::Tree;
use crate::ApplicationError;
use commit::Commit;

// Currently we are implementing only "loose objects". There are also "packfiles"
// but they are much more complex compared to loose objects. We can implement them
// in the future, but in that early stage, they are not worth implementing at all.
// We can live without them right now.
#[derive(Debug)]
pub enum Object {
    Blob(Blob),
    Commit(Commit),
    Tree(Tree),
    // Tag(), // Optional TODO: implement it later maybe?
}

impl Object {
    pub fn new(obj_type: &str, raw: &str) -> Result<Object, ApplicationError> {
        match obj_type {
            "blob" => Ok(Object::Blob(Blob::from_raw(raw)?)),
            "commit" => Ok(Object::Commit(Commit::from_raw(raw)?)),
            "tree" => Ok(Object::Tree(Tree::from_raw(raw)?)),
            _ => panic!("Unknown type!"), // TODO: return error
        }
    }

    pub fn serialize(&self) -> Result<String, ApplicationError> {
        match self {
            Object::Blob(blob) => blob.serialize(),
            Object::Commit(commit) => commit.serialize(),
            Object::Tree(tree) => tree.serialize(),
        }
    }

    // Underscore because `type` is a reserved keyword.
    pub fn type_(&self) -> &str {
        match self {
            Object::Blob(_) => "blob",
            Object::Commit(_) => "commit",
            Object::Tree(_) => "tree",
        }
    }
}
