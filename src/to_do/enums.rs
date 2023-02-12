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

impl From<String> for TaskStatus {
    fn from(status: String) -> Self {
        match status.as_str() {
            "PENDING" => TaskStatus::Pending,
            "COMPLETED" => TaskStatus::Completed,
            "DELAYED" => TaskStatus::Delayed,
            "ABANDONED" => TaskStatus::Abandoned,
            _ => panic!("Invalid status: {status} not supported"),
        }
    }
}
