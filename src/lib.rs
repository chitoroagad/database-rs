use rng::Rand;
use std::{
    env::temp_dir,
    ffi::OsString,
    fs::{self, OpenOptions},
    io::{self, ErrorKind, Write},
    os::unix::fs::OpenOptionsExt,
    time::{SystemTime, UNIX_EPOCH},
};

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

    fs::rename(tmp_path, path)?; // `mv tmp path`

    drop(file);

    fs::remove_file(temp_dir())?; // `rm tmp`

    Ok(())
}

#[cfg(test)]
mod tests {
}

