use crate::repository::Repository;
use clap::ArgMatches;
use log::info;
use std::error::Error;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, Write};

#[derive(Debug)]
pub enum InitError {
    AlreadyExists,
    NoDirectoryArg,
    IOError(io::ErrorKind),
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyExists => write!(
                f,
                "Init error: Given directory is not empty. Choose a different name."
            ),
            Self::NoDirectoryArg => write!(f, "Init error: Please give the directory name."),
            Self::IOError(kind) => write!(f, "Init error: {:?}", kind),
        }
    }
}

impl Error for InitError {
    fn description(&self) -> &str {
        match self {
            Self::AlreadyExists => {
                "Init error: Given directory is not empty. Choose a different name."
            }
            Self::NoDirectoryArg => "Init error: Please give the directory name.",
            Self::IOError(_kind) => "Init error: IOError", // TODO: print kind
        }
    }
}

impl From<io::Error> for InitError {
    fn from(error: io::Error) -> Self {
        let kind = error.kind();
        match kind {
            io::ErrorKind::AlreadyExists => InitError::AlreadyExists,
            err => InitError::IOError(err),
        }
    }
}

impl<'a> Repository<'a> {
    pub fn init(matches: &ArgMatches) -> Result<(), InitError> {
        info!("Running init sub command");

        // 1: Create repository directory.
        let dir_name = matches.value_of("directory").unwrap_or("");
        if dir_name == "" {
            // That shouldn't happen and clap should warn the user before here,
            // but checking just in case.
            return Err(InitError::NoDirectoryArg);
        }

        info!("Creating repo directory");
        create_dir(&dir_name)?;

        // 2: Create .gel dir.
        create_dir(&*format!("{}/.gel", dir_name))?;

        // 3. Create refs dir.
        info!("Creating refs");
        // Consider recursively creating heads and removing the first create_dir call.
        create_dir(&*format!("{}/.gel/refs", dir_name))?;
        create_dir(&*format!("{}/.gel/refs/heads", dir_name))?;
        create_dir(&*format!("{}/.gel/refs/tags", dir_name))?;

        info!("Creating HEAD");
        // 4. Create HEAD symlink to master: ref: refs/heads/master
        let mut head_file = File::create(&*format!("{}/.gel/HEAD", dir_name))?;
        head_file.write_all(b"ref: refs/heads/master")?;

        // 4. Create config file and fill it.
        create_and_fill_config(&dir_name)?;

        // 5. Create object directory.
        create_dir(&*format!("{}/.gel/objects", dir_name))?;
        create_dir(&*format!("{}/.gel/objects/info", dir_name))?;
        create_dir(&*format!("{}/.gel/objects/packs", dir_name))?;

        // Optional/Future TODO: Create hooks folder.
        // Optional/Future TODO: Create other arguments like --bare --quiet

        info!("Init complete");
        Ok(())
    }
}

fn create_and_fill_config(dir_name: &str) -> Result<(), InitError> {
    info!("Creating config");
    let mut config_buffer = File::create(&*format!("{}/.gel/config", dir_name))?;
    let config_string = "[core]\n    filemode = true\n    bare = false".to_string();
    config_buffer.write_all(config_string.as_bytes())?;

    Ok(())
}

fn create_dir(dir_name: &str) -> Result<(), InitError> {
    let result = fs::create_dir(dir_name);
    if let Err(err) = result {
        return Err(err.into());
    }
    Ok(())
}
