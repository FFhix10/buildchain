use std::collections::BTreeMap;
use std::fs::{File, read_dir};
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

use super::Sha384;

/// A manifest of build artifacts
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Manifest {
    /// The timestamp of the source control revision
    pub time: u64,
    /// A dictionary of filenames and their hashes
    pub files: BTreeMap<String, Sha384>,
}

impl Manifest {
    /// Create a new Manifest by reading the provided build directory
    ///
    /// # Arguments
    ///
    /// * `time` - the timestamp of the source control revision that was built
    /// * `path` - the directory containing the build artifacts
    ///
    /// # Return
    ///
    /// The Manifest of the provided build data
    ///
    /// # Errors
    ///
    /// Errors that are encountered while reading will be returned
    pub fn new<P: AsRef<Path>>(time: u64, path: P) -> Result<Manifest> {
        let mut files = BTreeMap::new();

        for entry_res in read_dir(path.as_ref())? {
            let entry = entry_res?;

            let name = entry.file_name().into_string().map_err(|_| {
                Error::new(ErrorKind::InvalidData, "Filename is not UTF-8")
            })?;

            let file = File::open(entry.path())?;
            let sha = Sha384::new(file)?;

            files.insert(name, sha);
        }

        Ok(Manifest {
            time: time,
            files: files,
        })
    }
}
