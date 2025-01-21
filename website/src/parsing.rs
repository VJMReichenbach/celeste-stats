use chrono::DateTime;
use serde::Deserialize;
use std::{fs, path::Path};
use toml::value::{Date, Time};

// TODO: make this configureable via Cargo.toml
const CUSTOM_MAP_LOCATION: &str = "../data/custom_maps/*.toml";

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
    let file_content = fs::read_to_string(path).unwrap();
    let map: CustomMap = toml::from_str(&file_content).unwrap();
    map
}

fn create_custom_map_table(maps: Vec<CustomMap>) -> String {
    // this is both the header and footer
    let head_foot = "<th>Title</th><th>Time</th><th>Deaths</th><th>Date cleared</th><th>Enjoyment</th><th>Difficulty</th><th>Notes</th>";
    let mut table = String::new();
    table += "<table class=\"table is-hoverable is-fullwidth\">";

    // add header
    table += "<thead><tr>";
    table += head_foot;
    table += "</tr></thead>";

    table += "<tbody>";

    // add rows
    for map in maps {
        table += "<tr>";
        table += &format!("<td>{}</td>", map.title);
        table += &format!("<td>{}</td>", map.time);
        table += &format!("<td>{}</td>", map.deaths);
        table += &format!("<td>{}</td>", map.date_cleared);
        table += &format!("<td>{}</td>", map.enjoyment);
        table += &format!("<td>{}</td>", map.difficulty);
        table += &format!("<td>{}</td>", map.notes);
        table += "</tr>";
    }

    table += "</tbody>";

    // add footer
    table += "<tfoot><tr>";
    table += head_foot;
    table += "</tr></tfoot>";

    table += "</table>";

    table
}

pub fn parse_custom_maps() -> (String, String, i32) {
    let mut total_seconds = 0; // in seconds
    let mut total_nanoseconds = 0; // additional nanoseconds
    let mut total_deaths = 0;
    let mut maps: Vec<CustomMap> = vec![];
    for path in glob::glob(CUSTOM_MAP_LOCATION).unwrap() {
        let path = path.unwrap();
        let map = parse_custom_map(&path);

        let sec = map.time.second as i64;
        let min = (map.time.minute as i64) * 60;
        let hour = (map.time.hour as i64) * 3600;

        total_seconds += sec + min + hour;
        total_nanoseconds += map.time.nanosecond;

        total_deaths += map.deaths;
        maps.push(map);
    }
    let total_time = DateTime::from_timestamp(total_seconds, total_nanoseconds).unwrap();
    let total_time = format!("Total time: {}", total_time.format("%H:%M:%S.%3f"));
    let table = create_custom_map_table(maps);
    (table, total_time, total_deaths)
}
