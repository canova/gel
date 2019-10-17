use crate::repository::Repository;
use crate::ApplicationError;
use clap::ArgMatches;
use log::info;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
pub enum LogError {
    //
}

impl Repository {
    pub fn log(&self, _matches: &ArgMatches) -> Result<(), ApplicationError> {
        info!("Running log sub command.");

        unimplemented!();
    }
}

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for LogError {
    fn description(&self) -> &str {
        // match self {
        //     Self::NoDirectoryArg => "Please give the directory name.",
        // }
        "TODO: implement"
    }
}
