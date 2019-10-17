use crate::objects::Object;
use crate::repository::Repository;
use crate::ApplicationError;
use clap::ArgMatches;
use log::info;
use std::fs::File;
use std::io::Read;

impl Repository {
    /// Gets the `file` argument and return the hash of that object.
    /// If `write` argument is set it also writes that object to repository folder.
    /// It's generally for debug purpose. If you are an end user, you will probably not need it
    pub fn hash_object(&self, matches: &ArgMatches) -> Result<(), ApplicationError> {
        info!("Running hash-object sub command.");
        let obj_type = matches.value_of("type").unwrap(); // default is blob from clap config
        let write = matches.is_present("write");
        let file = matches.value_of("file").unwrap(); // clap will return if file is not there.

        // FIXME: Doing lots of copying of this string. Fix it pls.
        // Reading the file content
        let mut f = File::open(file)?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;

        // Creating an object with the given file content
        let obj = Object::new(obj_type, &content)?;
        // writing the object(if write is true) and getting the has value
        let hash = self.write_object(&obj, write)?;
        println!("{}", hash.to_string());
        Ok(())
    }
}
