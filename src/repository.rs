use crate::hash::Hash;
use crate::objects::Object;
use crate::ApplicationError;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use log::info;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
pub enum RepositoryError {
    NotFound,
    FileFoundInsteadOfDir,
    DirFoundInsteadOfDir,
    DirExists,
    DirNotExists,
    FileNotExists,
}

#[derive(Debug)]
pub struct Repository {
    pub directory: PathBuf,
}

impl Repository {
    /// Creates a new Repository instance by checking if the current directory
    /// is inside a repository. Panics if current directory is not inside one.
    pub fn new() -> Result<Self, ApplicationError> {
        let mut directory = env::current_dir()?;
        get_repo_directory(&mut directory)?;
        Ok(Repository { directory })
    }

    #[allow(dead_code)]
    fn get_dir(&self, dir_path: &[&str], mkdir: bool) -> Result<PathBuf, ApplicationError> {
        let mut new_path = self.directory.clone();
        new_path.push(".gel");
        new_path.extend(dir_path);

        if new_path.exists() {
            if new_path.is_dir() {
                Ok(new_path)
            } else {
                Err(Box::new(RepositoryError::FileFoundInsteadOfDir))
            }
        } else if mkdir {
            fs::create_dir_all(&new_path)?;
            Ok(new_path)
        } else {
            Err(Box::new(RepositoryError::DirNotExists))
        }
    }

    fn get_file(&self, dir_path: &[&str], mkdir: bool) -> Result<PathBuf, ApplicationError> {
        info!("get file");
        let mut new_path = self.directory.clone();
        new_path.push(".gel");
        new_path.extend(dir_path);

        info!("new path {:?}", new_path);
        if new_path.exists() {
            info!("path exists");
            if !new_path.is_dir() {
                info!("path is not a dir");
                Ok(new_path)
            } else {
                Err(Box::new(RepositoryError::DirFoundInsteadOfDir))
            }
        } else if mkdir {
            info!("creating the dirs");
            let mut parent_dir = new_path.clone();
            parent_dir.pop();
            info!("parent dir: {:?}", parent_dir);
            fs::create_dir_all(parent_dir)?;
            // We don't need to create this here, we are going to do it after calling this function anyway.
            // Keeping this here as commented just in case for later changes.
            // fs::File::create(&new_path)?;

            Ok(new_path)
        } else {
            Err(Box::new(RepositoryError::FileNotExists))
        }
    }

    pub fn read_object(&self, hash: &Hash) -> Result<Object, ApplicationError> {
        let hash_parts = hash.to_string_parts();
        let file_path = self.get_file(&["objects", &hash_parts.0, &hash_parts.1], false)?;

        // Getting the file content
        info!("reading file path: {:?}", file_path);
        let mut file = fs::File::open(file_path)?;
        let mut content_bytes = Vec::new();
        file.read_to_end(&mut content_bytes)?;
        info!("Read the file content");

        // Decoding the encoded zlib data
        let mut decoder = ZlibDecoder::new(&content_bytes[..]);
        let mut raw = String::new();
        decoder.read_to_string(&mut raw)?;
        info!("Decoded data: {:?}", raw);

        // Reading the object type
        let x = raw.find(' ').unwrap(); // TODO: return friendly error
        info!("x: {:?}", x);
        let obj_type = &raw[..x];
        info!("Object type: {:?}", obj_type);

        let y = raw[x..]
            .as_bytes()
            .iter()
            .position(|&a| a == b'\x00')
            .map(|i| x + i)
            .unwrap();
        info!("y: {:?}", y);
        let size: usize = FromStr::from_str(&raw[(x + 1)..y])?;
        info!("obj size: {:?}", size);

        if size != raw.len() - y - 1 {
            panic!("Malformed object!"); // TODO: return an error.
        }

        info!("Creating the object now");
        Object::new(obj_type, &raw[(y + 1)..])
    }

    pub fn write_object(&self, obj: &Object, write: bool) -> Result<Hash, ApplicationError> {
        info!("Serializing the object");
        // File header
        // Serializing the object data first.
        let serialized_obj = obj.serialize()?;
        info!("Serialized object");

        // TODO: Consider moving this logic into its own function
        // Adding header to the serialized object.
        // Header is like this:
        // 1. object type
        // 2. a space char
        // 3. object size
        // 4. b'\x00' to represent the object size ending
        // 5. serialized objet
        let mut content_vec: Vec<u8> = Vec::new();
        content_vec.extend(obj.type_().as_bytes());
        content_vec.push(b' ');
        content_vec.extend(serialized_obj.len().to_string().as_bytes());
        content_vec.push(b'\x00');
        content_vec.extend(serialized_obj.as_bytes());

        info!("Serialized all the content");

        let hash = Hash::new(&std::str::from_utf8(&content_vec)?);

        if write {
            info!("Compressing the content");
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&content_vec)?;
            let compressed = encoder.finish()?;

            info!("Writing compressed content to file");
            let hash_parts = hash.to_string_parts();
            let path = self.get_file(&["objects", &hash_parts.0, &hash_parts.1], true)?;
            info!("path to write: {:?}", path);
            let mut f = fs::File::create(path)?;
            f.write_all(&compressed)?;
        }

        Ok(hash)
    }
}

/// Gets a mutable directory reference and checks if that directory contains a
/// `.gel` repository folder. If it does, returns Ok, if it doesn't it goes back
/// one directory and checks that one. If it tries to pop the root directory,
/// returns error.
fn get_repo_directory(directory: &mut PathBuf) -> Result<(), ApplicationError> {
    directory.push(".gel");
    if directory.is_dir() {
        directory.pop();
        return Ok(());
    }

    // Pop the .gel folder
    assert!(directory.pop());

    // Pop again to go one directory back.
    if directory.pop() {
        return get_repo_directory(directory);
    }

    Err(Box::new(RepositoryError::NotFound))
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for RepositoryError {
    fn description(&self) -> &str {
        match self {
            Self::NotFound => "Not a gel repository!",
            Self::FileFoundInsteadOfDir => "There is a file already in the given directory path.",
            Self::DirFoundInsteadOfDir => "There is a directory already in the given file path.",
            Self::DirExists => "Directory already exists.",
            Self::DirNotExists => "Directory does not exist.",
            Self::FileNotExists => "File does not exist.",
        }
    }
}
