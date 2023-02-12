use super::super::enums::TaskStatus;
use super::base::Base;

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
        Self {
            super_struct: base,
        }
    }
}