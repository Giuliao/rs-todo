use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

use crate::state::{read_file, write_to_file};

pub trait Delete {
    fn delete(&self, title: &String, status: &String, state: &mut Map<String, Value>) {
        state.remove(title);
        write_to_file("./state.json", state);
        println!("\n\n{} is being deleted\n\n", title);
    }
}
