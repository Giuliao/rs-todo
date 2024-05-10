mod todo;

use todo::enums::TaskStatus;
use todo::todo_factory;
use todo::ItemTypes;

use crate::todo::traits::delete::Delete;
use crate::todo::traits::edit::Edit;
use crate::todo::traits::get::Get;

fn main() {
    let todo_item = todo_factory("washing", TaskStatus::DONE);
    match todo_item {
        ItemTypes::Done(item) => {
            item.get(&item.super_struct.title);
            item.delete(&item.super_struct.title);
        }
        ItemTypes::Pending(item) => {
            item.get(&item.super_struct.title);
            item.set_to_done(&item.super_struct.title);
        }
    }
}
