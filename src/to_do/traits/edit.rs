use crate::{args::write_file, to_do::enums::TaskStatus};
use serde_json::{json, Map, Value};

pub trait Edit {
    fn change_to_completed(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(TaskStatus::Completed));
        write_file("./state.json", state);
        println!("\n\n{title}: changed to COMPLETED\n\n");
    }

    fn change_to_delayed(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(TaskStatus::Delayed));
        write_file("./state.json", state);
        println!("\n\n{title}: change to DELAYED\n\n");
    }

    fn change_to_abandoned(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(TaskStatus::Abandoned));
        write_file("./state.json", state);
        println!("\n\n{title}: change to ABANDONED\n\n");
    }

    fn change_to_pending(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(TaskStatus::Pending));
        write_file("./state.json", state);
        println!("\n\n{title}: change to PENDING\n\n");
    }
}
