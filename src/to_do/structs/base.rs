use super::super::enums::TaskStatus;

#[derive(Debug)]
pub struct Base {
    pub task: String,
    pub status: TaskStatus,
}
