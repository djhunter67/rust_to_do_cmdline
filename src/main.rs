mod args;

use args::ToDoArgs;
use clap::Parser;

fn main() {
    let args = ToDoArgs::parse();
    println!("{args:?}");
}
