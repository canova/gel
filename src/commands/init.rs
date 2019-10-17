use crate::repository::Repository;
use crate::ApplicationError;
use clap::ArgMatches;
use log::info;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::{self, File};
use std::io::Write;

#[derive(Debug)]
pub enum InitError {
    AlreadyExists,
    NoDirectoryArg,
}

impl Repository {
    /// Inits an empty gel repository.
    pub fn init(matches: &ArgMatches) -> Result<(), ApplicationError> {
        info!("Running init sub command");

        // 1: Create repository directory.
        let dir_name = matches.value_of("directory").unwrap_or("");
        if dir_name == "" {
            // That shouldn't happen and clap should warn the user before here,
            // but checking just in case.
            return Err(Box::new(InitError::NoDirectoryArg));
        }

        let mut directory = env::current_dir().unwrap();
        directory.push(&dir_name);

        info!("Creating repo directory");
        create_dir(&dir_name)?;

        // 2: Create .gel dir.
        create_dir(&*format!("{}/.gel", dir_name))?;

        // 3. Create .gel/refs dir.
        info!("Creating refs");
        // Consider recursively creating heads and removing the first create_dir call.
        create_dir(&*format!("{}/.gel/refs", dir_name))?;
        create_dir(&*format!("{}/.gel/refs/heads", dir_name))?;
        create_dir(&*format!("{}/.gel/refs/tags", dir_name))?;

        info!("Creating HEAD");
        // 4. Create HEAD symlink to master: "ref: refs/heads/master"
        let mut head_file = File::create(&*format!("{}/.gel/HEAD", dir_name))?;
        head_file.write_all(b"ref: refs/heads/master")?;

        // 4. Create config file and fill it.
        create_and_fill_config(&dir_name)?;

        // 5. Create object directory.
        create_dir(&*format!("{}/.gel/objects", dir_name))?;
        create_dir(&*format!("{}/.gel/objects/info", dir_name))?;
        create_dir(&*format!("{}/.gel/objects/packs", dir_name))?;

        // Optional/Future TODO: Create hooks folder.
        // Optional/Future TODO: Create other init arguments like --bare --quiet

        info!("Init complete");
        Ok(())
    }
}

fn create_and_fill_config(dir_name: &str) -> Result<(), ApplicationError> {
    info!("Creating config");
    let mut config_buffer = File::create(&*format!("{}/.gel/config", dir_name))?;
    // TODO: get some config arguments from init command and fill here depending on that.
    let config_string = "[core]\n    filemode = true\n    bare = false".to_string();
    config_buffer.write_all(config_string.as_bytes())?;

    Ok(())
}

fn create_dir(dir_name: &str) -> Result<(), ApplicationError> {
    Ok(fs::create_dir(dir_name)?)
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for InitError {
    fn description(&self) -> &str {
        match self {
            Self::AlreadyExists => "Given directory is not empty. Choose a different name.",
            Self::NoDirectoryArg => "Please give the directory name.",
        }
    }
}
