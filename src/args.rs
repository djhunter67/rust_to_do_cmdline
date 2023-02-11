use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about,
    long_about = "Command line to-do application w/ persistence"
)]
pub struct ToDoArgs {
    /// The status of the to do item
    /// DONE, STALLED, PENDING
    #[arg(long)]
    pub status: String,
    /// The task of the to do item
    /// "Do the dishes", "Finish the laundry"
    #[arg(long)]
    pub task: String,
}
