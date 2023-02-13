use super::super::enums::TaskStatus;

/// Base struct for all to do items
///
/// # Examples
/// ```
/// use to_do::enums::TaskStatus;
/// pub super_struct: Base,
///
/// impl Delayed {
///  pub fn new(task_title: &str) -> Self {
///     let base = Base {
///        task: task_title.to_string(),
///       status: TaskStatus::Delayed,
///    };
///   Self {
///     super_struct: base,
///  }
/// }
/// }
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
#[derive(Debug)]
pub struct Base {
    pub task: String,
    pub status: TaskStatus,
}
