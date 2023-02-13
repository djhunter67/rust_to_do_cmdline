use super::super::traits::delete::Delete;
use super::super::traits::edit::Edit;
use super::super::traits::get::Get;

use super::super::enums::TaskStatus;
use super::base::Base;

/// Abandoned struct for all to do items
/// # Examples
/// ```
/// fn process_abandoned(item: Abandoned, command: &str, state: &Map<String, Value>) {
///     let mut state = state.clone();
///
///     match command {
///        "get" => item.get(&item.super_struct.task, state),
///        "create" => item.create(
///            &item.super_struct.task,
///            &item.super_struct.status.to_string(),
///            &mut state,
///        ),
///        "edit" => item.change_to_abandoned(&item.super_struct.task, &mut state),
///        _ => println!("command: Abandoned - {command} - Not supported"),
///   }
/// }
/// ```
/// # Errors
/// ```shoud_error
///
/// let mut state = Map::new();
/// let item = Abandoned::new("test");
/// let command = "invalid";
/// let result = item.create(&item.super_struct.task, &item.super_struct.status.to_string(), &mut state);
/// ```
/// # Output
/// ```text
/// "command: Abandoned - {command} - Not supported"
/// ```
#[derive(Debug)]
pub struct Abandoned {
    pub super_struct: Base,
}

impl Abandoned {
    pub fn new(task_title: &str) -> Self {
        let base = Base {
            task: task_title.to_string(),
            status: TaskStatus::Abandoned,
        };
        Self { super_struct: base }
    }
}

impl Get for Abandoned {}
impl Delete for Abandoned {}
impl Edit for Abandoned {}
