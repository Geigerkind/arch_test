use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_file_content(file_path: &Path) -> String {
    let mut file = File::open(file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}