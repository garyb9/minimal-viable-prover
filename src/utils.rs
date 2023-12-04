use std::fs;

pub fn file_exists(file_path: &str) -> bool {
    fs::metadata(file_path).is_ok()
}
