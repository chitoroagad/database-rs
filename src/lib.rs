use rng::Rand;
use std::{
    ffi::OsString,
    fs::{self, OpenOptions},
    io::{self, ErrorKind, Write},
    os::unix::fs::OpenOptionsExt,
    time::{SystemTime, UNIX_EPOCH},
};

/// Saves `data` atomically to file at `path`
/// Atomically writes the given data to the specified file path.
///
/// This function writes `data` to a temporary file in the same location as `path`,
/// then atomically renames it to `path` to avoid partial writes. The temporary
/// file is uniquely named using a timestamp-based random number to avoid collisions.
///
/// # Arguments
///
/// - `path` - The final destination file path.
/// - `data` - A byte slice containing the data to be written.
///
/// # Returns
///
/// - `Ok(())` on success.
/// - `Err(io::Error)` if there is a problem writing, syncing, renaming the file,
///   or generating the temporary file.
///
/// # Errors
///
/// This function may return an error in the following cases:
/// - The provided path is invalid or not found.
/// - The file already exists and `create_new(true)` fails.
/// - There are insufficient permissions to write to the path.
/// - There is an I/O error during writing, syncing, or renaming.
///
/// # Notes
///
/// - The temporary file is created with `0o664` permissions.
/// - Uses `fs::rename` for atomic replacement.
/// - May log certain system errors to stderr.
///
/// # Example
///
/// ```no_run
/// use std::path::PathBuf;
/// use database_rs::save_data;
///
/// let path = PathBuf::from("/tmp/mydata.txt");
/// let data = b"important information";
/// save_data(path, data);
/// ```
pub fn save_data(path: std::path::PathBuf, data: &[u8]) -> Result<(), io::Error> {
    let mut rng = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Err(e) => {
            eprintln!("SystemTimeError: difference {:?}", e.duration());
            return Err(io::Error::new(ErrorKind::InvalidData, e));
        }
        Ok(d) => Rand::new(d.as_secs() as u32),
    };

    let tmp_path = OsString::from(format!(
        "{}.tmp.{}",
        path.as_os_str().to_string_lossy(),
        rng.rand_range(1, 9999)
    ));

    let mut file = match OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o664)
        .open(&tmp_path)
    {
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                eprintln!("Invalid Path");
                return Err(e);
            }
            ErrorKind::PermissionDenied => return Err(e),
            ErrorKind::AlreadyExists => return Err(e),
            _ => return Err(io::Error::other(e)),
        },
        Ok(f) => f,
    };

    file.write_all(data)?; // Save to temporary file
    file.sync_all()?; // fsync syscall
    fs::rename(&tmp_path, path)?; // `mv tmp path`
    drop(file);
    let _ = fs::remove_file(tmp_path); // `rm tmp` if it still exists
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::Read, path::PathBuf};

    #[test]
    fn save_data_success() {
        let file_path = PathBuf::from("/tmp/rust_test");
        let data = b"Hello, world!";

        let result = save_data(file_path.clone(), data);

        assert!(result.is_ok());

        let mut contents = Vec::new();
        let mut file = fs::File::open(file_path).expect("Failed to open written file");
        file.read_to_end(&mut contents)
            .expect("Failed to read file");
        assert_eq!(contents, data);
    }
}
