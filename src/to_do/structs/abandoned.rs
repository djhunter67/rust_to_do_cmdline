use super::super::traits::delete::Delete;
use super::super::traits::edit::Edit;
use super::super::traits::get::Get;

use super::super::enums::TaskStatus;
use super::base::Base;

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
        Self {
            super_struct: base,
        }
    }
}

impl Get for Abandoned {}
impl Delete for Abandoned {}
impl Edit for Abandoned {}