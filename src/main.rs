use std::path::PathBuf;

use database_rs::save_data;

fn main() {
    let _ = save_data(PathBuf::from("data.data"), "DATAT".as_bytes());
}
