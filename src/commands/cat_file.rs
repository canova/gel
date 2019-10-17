use crate::hash::Hash;
use crate::repository::Repository;
use crate::ApplicationError;
use clap::ArgMatches;
use log::info;

impl Repository {
    /// Gets the repository object from argument and returns the content of it.
    /// It's generally for debug purpose. If you are an end user, you will probably not need it
    pub fn cat_file(&self, matches: &ArgMatches) -> Result<(), ApplicationError> {
        info!("Running cat-file sub command.");
        let _obj_type = matches.value_of("type").unwrap(); // default is blob from clap config
        let hash_string = matches.value_of("object").unwrap(); // clap will throw error if object is not there.

        let hash = Hash::from_hex(hash_string);
        let object = self.read_object(&hash)?;
        println!("object: {}", object.serialize()?);
        Ok(())
    }
}
