use serde_json::value::Value;
use serde_json::Map;

pub trait Get {
    fn get(&self, title: &String, state: &Map<String, Value>) {
        let item: Option<&Value> = state.get(title);
        match item {
            Some(value) => println!("\n\nItem: {}", value.to_string()),
            None => println!("item: {} was not found", title),
        }
    }
}
