use crate::ApplicationError;

/// TODO: write and add some useful helper functions.
#[derive(Debug)]
pub struct User {
    name: String,
    email: String,
}

impl User {
    pub fn new(_user_string: &str) -> Result<User, ApplicationError> {
        unimplemented!()
    }
}
