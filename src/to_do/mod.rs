pub mod enums;
pub mod structs;
pub mod traits;

use enums::TaskStatus;
use structs::abandoned::Abandoned;
use structs::completed::Completed;
use structs::delayed::Delayed;
use structs::pending::Pending;

#[derive(Debug)]
pub enum ItemTypes {
    Pending(Pending),
    Completed(Completed),
    Delayed(Delayed),
    Abandoned(Abandoned),
}

pub fn to_do_factory(title: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::Pending => ItemTypes::Pending(Pending::new(title)),
        TaskStatus::Completed => ItemTypes::Completed(Completed::new(title)),
        TaskStatus::Delayed => ItemTypes::Delayed(Delayed::new(title)),
        TaskStatus::Abandoned => ItemTypes::Abandoned(Abandoned::new(title)),
    }
}
