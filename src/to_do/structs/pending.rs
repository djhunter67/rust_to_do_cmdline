use super::super::enums::TaskStatus;
use super::super::traits::create::Create;
use super::super::traits::edit::Edit;
use super::super::traits::get::Get;
use super::base::Base;

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
