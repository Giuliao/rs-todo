pub mod enums;
pub mod structs;

use enums::TaskStatus;
use structs::done::Done;
use structs::pending::Pending;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}

pub fn todo_factory(title: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::PENDING => ItemTypes::Pending(Pending::new(title)),
        TaskStatus::DONE => ItemTypes::Done(Done::new(title)),
    }
}
