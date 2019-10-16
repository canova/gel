use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Repository {
    directory: PathBuf,
}

impl Repository {
    pub fn new() -> Self {
        let mut directory = env::current_dir().unwrap();
        directory = match get_repo_directory(directory) {
            Some(dir) => dir,
            None => panic!("Not a gel repository."), // TODO: return Result<Self> maybe?
        };
        Repository { directory }
    }
}

fn get_repo_directory(mut directory: PathBuf) -> Option<PathBuf> {
    directory.push(".gel");
    if directory.is_dir() {
        directory.pop();
        return Some(directory);
    }

    // Pop the .gel folder
    directory.pop();

    // Pop again to go one directory back.
    if directory.pop() {
        return get_repo_directory(directory);
    }

    None
}
