use crate::to_do::structs::abandoned::Abandoned;
use crate::to_do::structs::completed::Completed;
use crate::to_do::structs::delayed::Delayed;
use crate::to_do::structs::pending::Pending;
use crate::to_do::traits::create::Create;
use crate::to_do::traits::delete::Delete;
use crate::to_do::traits::edit::Edit;
use crate::to_do::traits::get::Get;

use crate::to_do::ItemTypes;
use serde_json::{Map, Value};

fn process_pending(item: Pending, command: &str, state: &Map<String, Value>) {
    let mut state = state.clone();

    match command {
        "get" => item.get(&item.super_struct.task, state),
        "create" => item.create(
            &item.super_struct.task,
            &item.super_struct.status.to_string(),
            &mut state,
        ),
        "edit" => item.change_to_pending(&item.super_struct.task, &mut state),
        _ => println!("command: Pending - {command} - Not supported"),
    }
}

fn process_done(item: Completed, command: &str, state: &Map<String, Value>) {
    let mut state = state.clone();

    match command {
        "get" => item.get(&item.super_struct.task, state),

        "edit" => item.change_to_completed(&item.super_struct.task, &mut state),

        "delete" => item.delete(&item.super_struct.task, &mut state),
        _ => println!("command: Completed - {command} - Not supported"),
    }
}

fn process_abandoned(item: Abandoned, command: &str, state: &Map<String, Value>) {
    let mut state = state.clone();

    match command {
        "get" => item.get(&item.super_struct.task, state),

        "edit" => item.change_to_abandoned(&item.super_struct.task, &mut state),

        "delete" => item.delete(&item.super_struct.task, &mut state),
        _ => println!("command: Abandoned - {command} - Not supported"),
    }
}

fn process_delayed(item: Delayed, command: &str, state: &Map<String, Value>) {
    let mut state = state.clone();

    match command {
        "get" => item.get(&item.super_struct.task, state),

        "edit" => item.change_to_delayed(&item.super_struct.task, &mut state),

        _ => println!("command: Delayed - {command} - Not supported"),
    }
}

pub fn process_input(item: ItemTypes, command: &str, state: &Map<String, Value>) {
    match item {
        ItemTypes::Pending(pending) => process_pending(pending, command, state),
        ItemTypes::Completed(completed) => process_done(completed, command, state),
        ItemTypes::Delayed(delayed) => process_delayed(delayed, command, state),
        ItemTypes::Abandoned(abandoned) => process_abandoned(abandoned, command, state),
    }
}
