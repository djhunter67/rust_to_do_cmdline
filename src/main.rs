mod args;
mod to_do;

// use args::ToDoArgs;
// use clap::Parser;
use args::read_file;
use args::write_file;
use serde_json::value::Value;
use serde_json::{json, Map};
use std::env;
use to_do::{
    enums::TaskStatus,
    to_do_factory,
    traits::{edit::Edit, get::Get},
    ItemTypes,
};

fn main() {
    // let args = ToDoArgs::parse();
    // println!("{args:?}");

    let args: Vec<String> = env::args().collect();

    let status: String = args[1].clone();
    let task: String = args[2].clone();

    let mut state: Map<String, Value> = read_file("./state.json");

    println!("Before operation: {state:?}");

    state.insert(task, json!(status));

    println!("After operation: {state:?}");

    write_file("./state.json", &mut state);

    let to_do_item = to_do_factory("Laundry", TaskStatus::Completed);

    match to_do_item {
        ItemTypes::Pending(pending) => {
            pending.get(&pending.super_struct.task);
            pending.change_to_pending(&pending.super_struct.task);
        }
        ItemTypes::Completed(completed) => {
            completed.get(&completed.super_struct.task);
            completed.change_to_completed(&completed.super_struct.task);
        }
        ItemTypes::Delayed(delayed) => {
            delayed.get(&delayed.super_struct.task);
            delayed.change_to_delayed(&delayed.super_struct.task);
        }
        ItemTypes::Abandoned(abandoned) => {
            abandoned.get(&abandoned.super_struct.task);
            abandoned.change_to_abandoned(&abandoned.super_struct.task);
        }
    }
}
