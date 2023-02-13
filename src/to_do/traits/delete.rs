use crate::args::write_file;
use serde_json::{Map, Value};

/// Delete to_do items
///     
/// # Examples
/// ```
/// impl Delete for ToDo {}
/// ```
pub trait Delete {
    /// Delete the passed in title from the db
    ///     
    /// # Examples
    /// ```
    /// let mut state = Map::new();
    /// let item = ToDo::new("test");
    /// let command = "delete";
    /// let result = item.delete(&item.super_struct.task, &mut state);
    /// ```
    /// # Output
    /// ```text
    /// "
    ///
    /// {title}: DELETED
    ///
    /// "
    /// ```
    fn delete(&self, title: &str, state: &mut Map<String, Value>) {
        state.remove(title);

        write_file("./state.json", state);

        println!("\n\n{title}: DELETED\n\n");
    }
}
