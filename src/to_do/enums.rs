use serde::Serialize;

/// Define the possible states of a task
///
/// # Examples
/// ```
/// use to_do::enums::TaskStatus;
///
/// let status = TaskStatus::Pending;
/// assert_eq!(status.to_string(), "PENDING");
/// ```
#[derive(Debug, Serialize)]
pub enum TaskStatus {
    Pending,
    Completed,
    Delayed,
    Abandoned,
}
/// Implement the Display trait for TaskStatus
///
/// # Examples
/// ```
/// use to_do::enums::TaskStatus;
///
/// let status = TaskStatus::Pending;
/// println!("{status}");
/// assert_eq!(status.to_string(), "PENDING");
/// ```
/// # Output
/// ```text
/// PENDING
/// ```
/// # Panics
/// ```should_panic
/// use to_do::enums::TaskStatus;
/// let statuses = TaskStatus::Pending;
/// println!("{status}");
/// assert_eq!(status.to_string(), "PENDING");
/// ```
/// # Output
/// ```text
/// thread 'main' panicked at 'Invalid status: {status} not supported', src/to_do/enums.rs:56:17
/// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
/// ```
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

/// Implement the From trait for TaskStatus
///
/// # Examples
/// ```
/// use to_do::enums::TaskStatus;
///
/// let status = "pending";
/// let status = TaskStatus::from(status.to_uppercase()
/// assert_eq!(status.to_string(), "PENDING");
/// ```
/// # Panics
/// ```should_panic
/// use to_do::enums::TaskStatus;
///
/// let status = "invalid";
/// let status = TaskStatus::from(status.to_uppercase()
/// assert_eq!(status.to_string(), "PENDING");
/// ```
/// # Output
/// ```text
/// thread 'main' panicked at 'Invalid status: {status} not supported', src/to_do/enums.rs:56:17
/// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
/// ```
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
