use std::path::PathBuf;

mod b_tree;
use database_rs::save_data;

fn main() {
    let _ = save_data(PathBuf::from("data.data"), "DATAT".as_bytes());
}
