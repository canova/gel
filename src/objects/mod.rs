mod blob;
mod commit;
mod tree;

use crate::objects::blob::Blob;
use crate::objects::tree::Tree;
use crate::ApplicationError;
use commit::Commit;

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
            "blob" => Ok(Object::Blob(Blob::new(raw)?)),
            "commit" => Ok(Object::Commit(Commit::new(raw)?)),
            "tree" => Ok(Object::Tree(Tree::new(raw)?)),
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
