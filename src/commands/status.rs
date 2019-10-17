use crate::repository::Repository;
use crate::ApplicationError;
use clap::ArgMatches;
use log::info;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum StatusError {
    NoRepository,
}

impl Repository {
    /// TODO: write some docs
    pub fn status(&self, _matches: &ArgMatches) -> Result<(), ApplicationError> {
        info!("Running status sub command");

        unimplemented!()
    }
}

impl fmt::Display for StatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for StatusError {
    fn description(&self) -> &str {
        match self {
            Self::NoRepository => "There is no repository in the current directory.",
        }
    }
}
