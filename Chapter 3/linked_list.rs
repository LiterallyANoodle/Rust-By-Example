/*
	Linked list example 
*/

#[derive(Debug)]
enum List {
	// Cons: A tuple struct that wraps an element and a pointer to the next node
	Cons(i64, Box<List>),
	// Nil: marks the end of a list. Unit struct. 
	Nil,
}

fn main() {
	println!("{:?}", List::Cons(5, Box(List::Nil)));
}