use crate::{args::write_file, to_do::enums::TaskStatus};
use serde_json::{json, Map, Value};

/// Edit the passed in title from the db
///
/// # Examples
/// ```
/// impl Edit for Delayed {}
/// ```
pub trait Edit {
    /// Change the passed in title to completed
    ///
    /// # Examples
    /// ```
    /// let mut state = Map::new();
    /// let item = Delayed::new("test");
    /// let command = "completed";
    /// let result = item.edit(&item.super_struct.task, &item.super_struct.status.to_string(), &mut state);
    /// ```
    /// # Output
    /// ```text
    /// "
    ///
    /// {title}: changed to COMPLETED
    ///
    /// "
    /// ```
    fn change_to_completed(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(TaskStatus::Completed.to_string()));

        write_file("./state.json", state);

        println!("\n\n{title}: changed to COMPLETED\n\n");
    }

    /// Change the passed in title to delayed
    /// # Examples
    ///
    /// ```
    /// let mut state = Map::new();
    /// let item = Delayed::new("test");
    /// let command = "delayed";
    /// let result = item.edit(&item.super_struct.task, &item.super_struct.status.to_string(), &mut state);
    /// ```
    /// # Output
    /// ```text
    /// "
    ///
    /// {title}: change to DELAYED
    ///
    /// "
    /// ```
    fn change_to_delayed(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(TaskStatus::Delayed.to_string()));

        write_file("./state.json", state);
        println!("\n\n{title}: change to DELAYED\n\n");
    }

    /// Change the passed in title to abandoned
    /// # Examples
    ///
    /// ```
    /// let mut state = Map::new();
    /// let item = Delayed::new("test");
    /// let command = "abandoned";
    /// let result = item.edit(&item.super_struct.task, &item.super_struct.status.to_string(), &mut state);
    /// ```
    /// # Output
    /// ```text
    /// "
    ///
    /// {title}: change to ABANDONED
    ///
    /// "
    /// ```
    fn change_to_abandoned(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(TaskStatus::Abandoned.to_string()));

        write_file("./state.json", state);
        println!("\n\n{title}: change to ABANDONED\n\n");
    }

    /// Change the passed in title to pending
    /// # Examples
    ///
    /// ```
    /// let mut state = Map::new();
    /// let item = Delayed::new("test");
    /// let command = "pending";
    /// let result = item.edit(&item.super_struct.task, &item.super_struct.status.to_string(), &mut state);
    /// ```
    /// # Output
    /// ```text
    /// "
    ///
    /// {title}: change to PENDING
    ///
    /// "
    /// ```
    fn change_to_pending(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(TaskStatus::Pending.to_string()));

        write_file("./state.json", state);
        println!("\n\n{title}: change to PENDING\n\n");
    }
}
