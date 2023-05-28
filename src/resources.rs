// Import dependencies
use std::ffi;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

// Enum which holds all the error's that can occur
#[derive(Debug, Fail)] // Dervice Fail, in addition to Debug which is derived by default
pub enum Error {
    #[fail(display = "I/O error")]
    Io(#[cause] io::Error),
    #[fail(display = "Failed to read CString from file that contains 0")]
    FileContainsNil,
    #[fail(display = "Failed get executable path")]
    FailedToGetExePath,
}

// Resources struct
pub struct Resources {
    root_path: PathBuf,
}

// Implementation of Resources struct
impl Resources {
    pub fn from_relative_exe_path(rel_path: &Path) -> Result<Resources, Error> {
        // Get path to executable
        let exe_file_name = ::std::env::current_exe().map_err(|_| Error::FailedToGetExePath)?;
        // Get path to directory containing executable
        let exe_path = exe_file_name.parent().ok_or(Error::FailedToGetExePath)?;
        // Get path to resources directory
        let res_path = exe_path.join(rel_path);

        Ok(Resources {
            root_path: res_path,
        })
    }

    // Load a resource into a byte buffer
    pub fn load_cstring(&self, resource_name: &str) -> Result<ffi::CString, Error> {
        // Open file
        let mut file = fs::File::open(resource_name_to_path(&self.root_path, resource_name))?;

        // allocate buffer of the same size as file
        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);
        file.read_to_end(&mut buffer)?;

        // check for nul byte
        if buffer.iter().find(|i| **i == 0).is_some() {
            return Err(Error::FileContainsNil);
        }

        Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) })
    }
}

// Implement From trait for Error enum
impl From<io::Error> for Error {
    // Convert io::Error to Error
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

// Convert resource name to path
fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/") {
        path = path.join(part);
    }

    path
}
