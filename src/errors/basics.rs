// basics.rs

extern crate todolist_parser;

use todolist_parser::TodoList;

fn main() {
	let todos = TodoList::get_todos("example/todos");
	match todos {
		Ok(list) => println!("{:?}", list),
		Err(e) => {
			println!("{}", e.description());
			println!("{:?}", e);
		}
	}
}