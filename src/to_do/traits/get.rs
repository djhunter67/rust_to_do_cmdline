use serde_json::{Map, Value};

/// Get the passed in title from the db
///
/// # Examples
/// ```
/// impl Get for Delayed {}
/// ```
/// # Errors
/// ```shoud_error
/// let mut state = Map::new();
/// let item = Delayed::new("test");
/// let command = "invalid";
/// let result = item.create(&item.super_struct.task, &item.super_struct.status.to_string(), &mut state);
/// ```
/// # Output
/// ```text
/// "{title}: Not found"
/// ```
pub trait Get {
    fn get(&self, title: &str, state: Map<String, Value>) {
        let item: Option<&Value> = state.get(title);

        match item {
            Some(result) => println!("\n\n{title}:\n {result}"),
            None => println!("{title}: Not found"),
        }
    }
}
