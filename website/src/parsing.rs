use std::{error::Error, fs, path::Path};

// TODO: make this configureable via Cargo.toml
const data_location: &str = "../data/";

fn read_csv_file(p: &str) -> Result<String, Box<dyn Error>> {
    let file = data_location.to_owned() + p;
    let content: String = fs::read_to_string(file)?;

    Ok(content)
}

pub fn parse_csv_file(p: &str) -> String {
    let mut message = String::from("");
    let path = data_location.to_owned() + p;
    if let Ok(content) = read_csv_file(&path) {
        message = content
    } else {
        message = String::from("There was an error reading file: ") + &path
    }
    message
}
