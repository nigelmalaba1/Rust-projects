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
