use crate::repository::Repository;
use clap::ArgMatches;
use log::info;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum LogError {
    //
}

impl fmt::Display for LogError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // match self {
        //     Self::NoRepository => write!(f, "There is no repository in the current directory."),
        // }
        Ok(())
    }
}

impl Error for LogError {
    fn description(&self) -> &str {
        // match self {
        //     Self::NoDirectoryArg => "Init error: Please give the directory name.",
        // }
        "TODO: implement"
    }
}



impl Repository {
    pub fn log(&self, _matches: &ArgMatches) -> Result<(), LogError> {
        info!("Running log sub command.");

        unimplemented!();
    }
}