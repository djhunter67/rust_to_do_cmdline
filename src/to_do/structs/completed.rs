use super::super::traits::delete::Delete;
use super::super::traits::edit::Edit;
use super::super::traits::get::Get;

use super::super::enums::TaskStatus;
use super::base::Base;

/// Completed struct for all to do items
/// # Examples
/// ```
/// fn process_completed(item: Completed, command: &str, state: &Map<String, Value>) {
///     let mut state = state.clone();
///
///     match command {
///         "get" => item.get(&item.super_struct.task, state),
///         "create" => item.create(
///             &item.super_struct.task,
///             &item.super_struct.status.to_string(),
///             &mut state,
///         ),
///         "edit" => item.change_to_completed(&item.super_struct.task, &mut state),
///         _ => println!("command: Completed - {command} - Not supported"),
///     }
/// }
/// ```
/// # Errors
/// ```shoud_error
///
/// let mut state = Map::new();
/// let item = Completed::new("test");
/// let command = "invalid";
/// let result = item.create(&item.super_struct.task, &item.super_struct.status.to_string(), &mut state);
/// ```
/// # Output
/// ```text
/// "command: Completed - {command} - Not supported"
/// ```
#[derive(Debug)]
pub struct Completed {
    pub super_struct: Base,
}

impl Completed {
    pub fn new(task_title: &str) -> Self {
        let base = Base {
            task: task_title.to_string(),
            status: TaskStatus::Completed,
        };
        Self { super_struct: base }
    }
}

impl Get for Completed {}
impl Delete for Completed {}
impl Edit for Completed {}
