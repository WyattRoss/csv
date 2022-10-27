use std::error::Error;
use serde::Deserialize;
use csv::ReaderBuilder;

fn read(file_path: String) -> Result<(), Box<dyn Error>>{
    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .from_path(file_path)?;
    for result in reader.deserialize() {
        let entry: Entry = result?;
        dbg!("{:?}", entry);
    }
    Ok(())
}

fn main() {
    let file_path = "Enrollments.csv";
    read(file_path.to_string()).expect("Error :)");
}

#[derive(Debug, Deserialize)]
struct Entry {
    _timestamp: String,
    _preferred_name: String,
    _last_name: String,
    _rcs_id: String,
    _discord_id: String,
    _project: String,
    _graduation_year: i32,
}
