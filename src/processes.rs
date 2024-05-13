use serde_json::value::Value;
use serde_json::Map;

use super::todo::structs::done::Done;
use super::todo::structs::pending::Pending;
use super::todo::ItemTypes;

use super::todo::traits::create::Create;
use super::todo::traits::delete::Delete;
use super::todo::traits::edit::Edit;
use super::todo::traits::get::Get;

pub fn processing_pending(item: Pending, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "create" => item.create(
            &item.super_struct.title,
            &item.super_struct.status.stringify(),
            &mut state,
        ),
        "delete" => item.delete(&item.super_struct.title, &mut state),
        "get" => item.get(&item.super_struct.title, &state),
        "edit" => item.set_to_done(&item.super_struct.title, &mut state),
        _ => println!("Invalid command"),
    }
}

pub fn processing_done(item: Done, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "delete" => item.delete(&item.super_struct.title, &mut state),
        "get" => item.get(&item.super_struct.title, &state),
        "edit" => item.set_to_pending(&item.super_struct.title, &mut state),
        _ => println!("Invalid command"),
    }
}

pub fn process_input(item: ItemTypes, command: String, state: &Map<String, Value>) {
    match item {
        ItemTypes::Pending(item) => processing_pending(item, command, state),
        ItemTypes::Done(item) => processing_done(item, command, state),
    }
}
