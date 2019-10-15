use clap::ArgMatches;
use log::info;
use std::error::Error;
use std::fmt;

// TODO: update errors
#[derive(Debug)]
pub enum StatusError {
    NoRepository,
}

impl fmt::Display for StatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoRepository => write!(f, "There is no repository in the current directory."),
        }
    }
}

impl Error for StatusError {
    fn description(&self) -> &str {
        "There is no repository in the current directory."
    }
}

/// TODO: write some docs
pub fn run_status(_matches: &ArgMatches) -> Result<(), StatusError> {
    info!("Running status sub command");

    unimplemented!()
}
