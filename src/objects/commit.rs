use crate::hash::Hash;
use crate::key_value_list::KVLIterator;
use crate::user::User;
use crate::ApplicationError;
use chrono::DateTime;
use chrono::FixedOffset;

#[derive(Debug)]
pub struct Commit {
    id: Hash,
    // Future TODO: add a committer too.
    author: User,
    // TODO: I think we should move this inside User, since commit date is attached to the author data.
    date: DateTime<FixedOffset>, // we should use to_rfc2822/parse_from_rfc2822
    message: String,
    // Every commit has a parent except the first commit.
    // Future TODO: Merge commits can have multiple parents.
    // Currently we don't have merge commit concept, but may have in the future.
    parent: Option<Hash>,
    tree: Hash,
}

impl Commit {
    /// Constructs Commit from raw file data.
    /// Commits have some key value pair list in the beginning and then have the message.
    /// It's stored like this (note the space before each key and \n after each pair):
    /// commit <hash>\n
    /// parent <hash>\n
    /// author <author-name>\n
    /// date <date>\n
    /// \n
    /// <commit-message>
    pub fn from_raw(raw: &str) -> Result<Commit, ApplicationError> {
        // this string here is  for testing purpose.
        // let s = "commit 11cb975741c78f17b34911bc08e5e14bc8e2e92a\nparent asdasdasd7b34911bc08e5e14bc8e2e9bb\nauthor asd\n desd\n hello\ndate canova\n\nmessageee yeeeeyyyy\nasdad aasdsd";
        let mut iter = KVLIterator::new(raw);
        let commit = iter
            .next()
            .expect("Commit pair expected in the commit object");
        assert!(
            commit.key == "commit",
            "Commit pair was expected but something else found"
        );
        let tree = iter
            .next()
            .expect("Parent pair expected in the commit object");
        assert!(
            tree.key == "tree",
            "Tree pair was expected but something else found"
        );
        let parent = iter
            .next()
            .expect("Parent pair expected in the commit object");
        assert!(
            parent.key == "parent",
            "Parent pair was expected but something else found"
        );
        let author = iter
            .next()
            .expect("Author pair expected in the commit object");
        assert!(
            author.key == "author",
            "Author pair was expected but something else found"
        );
        let date = iter
            .next()
            .expect("Date pair expected in the commit object");
        assert!(
            date.key == "date",
            "Date pair was expected but something else found"
        );
        let message = iter.next().expect("Message expected in the commit object");
        assert!(
            message.key == "",
            "Message pair was expected but something else found"
        );

        let last = iter.next();
        assert!(
            last.is_none(),
            "Unexpected content found inside commit object"
        );

        Ok(Commit {
            id: Hash::from_hex(commit.value)?,
            tree: Hash::from_hex(tree.value)?,
            parent: Hash::from_hex(parent.value).ok(),
            author: User::new(author.value)?,
            date: DateTime::parse_from_rfc2822(date.value)?,
            message: message.value.to_string(),
        })
    }

    pub fn serialize(&self) -> Result<String, ApplicationError> {
        unimplemented!()
    }
}
