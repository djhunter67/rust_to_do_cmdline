use super::super::enums::TaskStatus;
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
