use glob::glob;
use serde::Deserialize;
use std::{error::Error, fs, path::Path, time::Duration};
use toml::value::{Date, Time};
use toml::Value;

// TODO: make this configureable via Cargo.toml
const custom_map_location: &str = "../data/custom_maps/*.toml";

#[derive(Deserialize)]
pub struct CustomMap {
    title: String,
    time: Time,
    deaths: i32,
    date_cleared: Date,
    enjoyment: String,
    difficulty: String,
    notes: String,
}

fn parse_custom_map(path: &Path) -> CustomMap {
    // TODO: error here
    let file_content = fs::read_to_string(path).unwrap();
    let map: CustomMap = toml::from_str(&file_content).unwrap();
    map
}

pub fn parse_custom_maps() -> String {
    let mut maps: Vec<CustomMap> = vec![];
    for path in glob::glob(custom_map_location).unwrap() {
        let path = path.unwrap();
        let map = parse_custom_map(&path);
        maps.push(map);
    }

    let mut return_str = String::new();
    for c in maps {
        return_str += &format!("<li>{}</li>\n", c.title);
    }
    return_str
}
