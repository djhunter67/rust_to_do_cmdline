mod args;
mod to_do;

// use args::ToDoArgs;
// use clap::Parser;
use to_do::{enums::TaskStatus, to_do_factory, ItemTypes};

fn main() {
    // let args = ToDoArgs::parse();
    // println!("{args:?}");

    let to_do_item = to_do_factory("Laundry", TaskStatus::Abandoned);

    match to_do_item {
        ItemTypes::Pending(pending) => {
            println!("{:?}", pending.super_struct.task);
            println!("{:?}", pending.super_struct.status);
        }
        ItemTypes::Completed(completed) => {
            println!("{:?}", completed.super_struct.task);
            println!("{:?}", completed.super_struct.status);
        }
        ItemTypes::Delayed(delayed) => {
            println!("{:?}", delayed.super_struct.task);
            println!("{:?}", delayed.super_struct.status);
        }
        ItemTypes::Abandoned(abandoned) => {
            println!("{:?}", abandoned.super_struct.task);
            println!("{:?}", abandoned.super_struct.status);
        }
    }
}
