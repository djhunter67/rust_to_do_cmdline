use serde::Serialize;


#[derive(Debug, Serialize)]
pub enum TaskStatus {
    Pending,
    Completed,
    Delayed,
    Abandoned,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "PENDING"),
            TaskStatus::Completed => write!(f, "COMPLETED"),
            TaskStatus::Delayed => write!(f, "DELAYED"),
            TaskStatus::Abandoned => write!(f, "ABANDONED"),
        }
    }
}
