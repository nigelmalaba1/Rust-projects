use std::mem;

pub enum List<T> {
    Empty,
    Link(T, Box<List<T>>),
}

impl<T> List<T> {
    pub fn new() -> Self {
        List::Empty
    }

    pub fn push_front(&mut self, elem: T) {
        let new_node = Box::new(mem::replace(self, List::Empty));
        *self = List::Link(elem, new_node);
    }

    pub fn reverse(&mut self) {
        let mut current_node = List::Empty;
        while let List::Link(elem, next_node) = mem::replace(self, List::Empty) {
            current_node = List::Link(elem, Box::new(mem::replace(&mut current_node, List::Empty)));
            *self = mem::replace(&mut *next_node, List::Empty);
        }
        *self = current_node;
    }
}

fn main() {
    let mut list = List::new();

    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    list.push_front(4);
    list.push_front(5);

    println!("Original list: {:?}", list);
    list.reverse();
    println!("Reversed list: {:?}", list);
}
