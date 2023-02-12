use serde_json::{Map, Value};

pub trait Get {
    fn get(&self, title: &str, state: Map<String, Value>) {
        let item: Option<&Value> = state.get(title);

        match item {
            Some(item) => println!("\n\n{title}:\n {item}\n\n"),
            None => println!("{title}: Not found"),
        }
    }
}
