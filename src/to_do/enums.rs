#[derive(Debug)]
pub enum TaskSatatus {
    PENDING,
    COMPLETED,
}

impl std::fmt::Display for TaskSatatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TaskSatatus::PENDING => write!(f, "PENDING"),
            TaskSatatus::COMPLETED => write!(f, "COMPLETED"),
        }
    }
}
