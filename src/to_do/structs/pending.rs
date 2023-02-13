use super::super::enums::TaskStatus;
use super::super::traits::create::Create;
use super::super::traits::edit::Edit;
use super::super::traits::get::Get;
use super::base::Base;

/// Pending struct for all to do items
/// # Examples
/// ```
/// use to_do::enums::TaskStatus;
/// use to_do::traits::create::Create;
/// use to_do::traits::edit::Edit;
/// use to_do::traits::get::Get;
/// use to_do::structs::base::Base;
/// use to_do::structs::pending::Pending;
///
/// fn process_pending(item: Pending, command: &str, state: &Map<String, Value>) {
///     let mut state = state.clone();
///
///     match command {
///        "get" => item.get(&item.super_struct.task, state),
///        "create" => item.create(
///            &item.super_struct.task,
///            &item.super_struct.status.to_string(),
///            &mut state,
///        ),
///        "edit" => item.change_to_pending(&item.super_struct.task, &mut state),
///        _ => println!("command: Pending - {command} - Not supported"),
///   }
/// }
///
/// ```
/// # Errors
/// ```shoud_error
///
/// let mut state = Map::new();
/// let item = Pending::new("test");
/// let command = "invalid";
/// let result = item.create(&item.super_struct.task, &item.super_struct.status.to_string(), &mut state);
/// ```
/// # Output
/// ```text
/// "command: Pending - {command} - Not supported"
/// ```
#[derive(Debug)]
pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(task_title: &str) -> Self {
        let base = Base {
            task: task_title.to_string(),
            status: TaskStatus::Pending,
        };
        Self { super_struct: base }
    }
}

impl Get for Pending {}
impl Edit for Pending {}
impl Create for Pending {}
