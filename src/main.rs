extern crate rustd;

fn main() {
    let names = LinkedList::new();
    let names = names.add("Olakunle");
    let names = names.add("Arewa");
    let names = names.add("Daniel");
    println!("{:?}", names);
}

#[derive(Debug)]
enum LinkedList<T> {
    Cons(T, Box<LinkedList<T>>),
    Empty
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList::Empty
    }

    fn add(self, value: T) -> LinkedList<T> {
        LinkedList::Cons(value, Box::new(self))
    }
}
