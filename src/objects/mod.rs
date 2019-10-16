mod blob;
mod commit;
mod tree;


#[derive(Debug)]
pub enum Object {
    Blob(blob::Blob),
    Commit(commit::Commit),
    Tree(tree:Tree),
    // Tag(), // Optional TODO: implement it later maybe?
}