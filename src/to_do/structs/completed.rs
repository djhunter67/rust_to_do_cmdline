
use super::super::traits::get::Get;
use super::super::traits::delete::Delete;
use super::super::traits::edit::Edit;

use super::super::enums::TaskStatus;
use super::base::Base;

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
        Self {
            super_struct: base,
        }
    }
}

impl Get for Completed {}
impl Delete for Completed {}
impl Edit for Completed {}
