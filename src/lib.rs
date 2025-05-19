use std::{ffi::OsString, io, path::PathBuf};

pub fn save_data(path: std::path::PathBuf, data: &[u8]) -> Result<bool, io::Error> {
    tmp_path = OsString::from_str(format!("{}.tmp.{}", path, random))
}
