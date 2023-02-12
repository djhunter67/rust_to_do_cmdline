mod args;
mod processes;
mod to_do;

// use args::ToDoArgs;
// use clap::Parser;
use args::read_file;
use processes::process_input;
use serde_json::value::Value;
use serde_json::Map;
use std::env;
use to_do::{enums::TaskStatus, to_do_factory};

use crate::to_do::ItemTypes;

fn main() {
    // let args = ToDoArgs::parse();
    // println!("{args:?}");

    let args: Vec<String> = env::args().collect();

    let mut command: &str = &args[1];
    let title: &String = &args[2];

    let state: Map<String, Value> = read_file("./state.json");

    // let status: String = match command.as_str() {
    //     "create" => "Pending".to_string(),
    //     "edit" => "Pending".to_string(),
    //     "delete" => "Completed".to_string(),
    //     "get" => "Pending".to_string(),
    //     "abandoned" => "Abandoned".to_string(),
    //     "delayed" => "Delayed".to_string(),
    //     _ => "Pending".to_string(),
    // };

    let status = match &state.get(title) {
        Some(result) => result.to_string().replace('\"', ""),
        None => "pending".to_owned(),
    };

    let edit = String::from("edit");
    let delayed = String::from("delayed");
    let abandoned = String::from("abandoned");
    let completed = String::from("completed");
    let delete = String::from("delete");

    let item: ItemTypes = match command {
        "edit" => to_do_factory(title, TaskStatus::from(status.to_uppercase())),

        "delayed" => {
            command = edit.as_str();
            to_do_factory(title, TaskStatus::from(delayed.to_uppercase()))
        }

        "abandoned" => {
            command = edit.as_str();
            to_do_factory(title, TaskStatus::from(abandoned.to_uppercase()))
        }

        "completed" => {
            command = edit.as_str();
            to_do_factory(title, TaskStatus::from(completed.to_uppercase()))
        }

        "delete" => {
            command = delete.as_str();
            to_do_factory(title, TaskStatus::from(completed.to_uppercase()))
        }

        _ => to_do_factory(title, TaskStatus::from(status.to_uppercase())),
    };

    // let status = "Completed";
    println!("STATUS: {status}");

    // let item = to_do_factory(title, TaskStatus::from(status.to_uppercase()));

    println!("ITEM: {item:?}");

    process_input(item, command, &state);
}
