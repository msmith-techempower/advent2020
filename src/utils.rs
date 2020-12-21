use std::fs::File;
use std::io::Read;

pub fn read_input(file: &str) -> String {
    let mut file = File::open(file).expect(&format!("File exists and can be opened: {}", file));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Can read and is valid text");

    contents
}
