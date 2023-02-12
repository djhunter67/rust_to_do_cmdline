use std::fs;
use std::fs::File;
use std::io::Read;

use clap::Parser;
use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about,
    long_about = "Command line to-do application w/ persistence"
)]
pub struct ToDoArgs {
    /// The status of the to do item
    /// DONE, STALLED, PENDING
    #[arg(long)]
    pub status: String,
    /// The task of the to do item
    /// "Do the dishes", "Finish the laundry"
    #[arg(long)]
    pub task: String,
}

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = File::open(file_name).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    let json: Value = serde_json::from_str(&contents).expect("JSON was not well-formatted");
    json.as_object().unwrap().clone()
}

pub fn write_file(file_name: &str, state: &mut Map<String, Value>) {
    let json_string = json!(&state);

    fs::write(file_name, json_string.to_string()).expect("Unable to write file");
}
