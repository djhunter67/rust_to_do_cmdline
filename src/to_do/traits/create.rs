use serde_json::{json, Map, Value};

use crate::args::write_file;

/// Create new to_do items
///
/// # Examples
/// ```
/// impl Create for ToDo {}
/// ```
pub trait Create {
    /// Create a new to_do item
    ///     
    /// # Examples
    /// ```
    /// let mut state = Map::new();
    /// let item = ToDo::new("test");
    /// let command = "create";
    /// let result = item.create(&item.super_struct.task, &item.super_struct.status.to_string(), &mut state);
    /// ```
    /// # Output
    /// ```text
    /// "
    ///
    /// {title}: CREATED
    ///
    /// "
    /// ```
    fn create(&self, title: &str, status: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(status));
        write_file("./state.json", state);
        println!("\n\n{title}: CREATED\n\n");
    }
}
