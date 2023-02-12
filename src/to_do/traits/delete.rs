use crate::args::write_file;
use serde_json::{Map, Value};

pub trait Delete {
    fn delete(&self, title: &str, state: &mut Map<String, Value>) {
        
        state.remove(title);

        write_file("./state.json", state);
        
        println!("\n\n{title}: DELETED\n\n");
    }
}
