use serde_json::{json, Map, Value};

use crate::args::write_file;

pub trait Create {
    fn create(&self, title: &str, status: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(status));
        write_file("./state.json", state);
        println!("\n\n{title}: CREATED\n\n");
    }
}
