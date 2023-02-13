use super::super::traits::edit::Edit;
use super::super::traits::get::Get;

use super::super::enums::TaskStatus;
use super::base::Base;

/// Delayed struct for all to do items
///
/// # Examples
/// ```
/// use to_do::enums::TaskStatus;
/// pub super_struct: Base,
///
/// fn process_delayed(item: Delayed, command: &str, state: &Map<String, Value>) {
///     let mut state = state.clone();
///
///     match command {
///         "get" => item.get(&item.super_struct.task, state),
///
///         "edit" => item.change_to_delayed(&item.super_struct.task, &mut state),
///
///         _ => println!("command: Delayed - {command} - Not supported"),
///     }
/// }
#[derive(Debug)]
pub struct Delayed {
    pub super_struct: Base,
}

impl Delayed {
    pub fn new(task_title: &str) -> Self {
        let base = Base {
            task: task_title.to_string(),
            status: TaskStatus::Delayed,
        };
        Self { super_struct: base }
    }
}

impl Get for Delayed {}
impl Edit for Delayed {}
