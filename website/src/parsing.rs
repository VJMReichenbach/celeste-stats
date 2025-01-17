use std::{error::Error, fs, path::Path};

// TODO: make this configureable via Cargo.toml
const data_location: &str = "../data/";

fn read_csv_file(p: &str) -> Result<String, Box<dyn Error>> {
    let file = data_location.to_owned() + p;
    let content: String = fs::read_to_string(file)?;
    let mut table: String = String::from(
        "<div class=\"table-container\"><table class=\"table is-bordered is-hoverable\">",
    );
    let mut header: String = String::from("");
    let mut table_body = String::from("");
    for (i, line) in content.split("\n").enumerate() {
        let cells: Vec<&str> = line.split(",").collect();
        let status = cells[5];
        if status.trim() == "TODO" {
            table_body += "<tr class=\"is-danger\">";
        } else if status.trim() == "WIP" {
            table_body += "<tr class=\"is-warning\">";
        } else if status.trim() == "Completed" {
            table_body += "<tr class=\"is-success\">";
        } else {
            table_body += "<tr>";
        }

        for cell in cells {
            if i == 0 {
                header += "<th>";
                header += cell;
                header += "</th>";
            } else {
                table_body += "<td>";
                table_body += cell;
                table_body += "</td>";
            }
        }
        table_body += "</tr>";
    }
    table += &("<thead>".to_owned() + &header + "</thread>");
    table += &("<tfoot>".to_owned() + &header + "</tfoot>");
    table += &("<tbody>".to_owned() + &table_body + "</tbody>");
    table += "</table></div>";

    Ok(table)
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
