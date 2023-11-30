use std::{rc::Rc, cell::RefCell, ops::{DerefMut, Deref}};

#[derive(Clone)]
pub struct Cell<T> {
    succ: Link<T>,
    value: T
}

type Link<T> = Option<Rc<RefCell<Cell<T>>>>;

impl<T> Cell<T> {
    fn new(value: T) -> Rc<RefCell<Cell<T>>> {
        Rc::new(RefCell::new(Cell {
            succ: None,
            value: value,
        }))
    }
}

#[derive(Clone)]
pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    count: i32
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            count: 0
        }
    }

    pub fn add_cell(&mut self, value: T) {
        let new = Cell::new(value);

        match self.tail.take() {
            Some(old) => old.borrow_mut().succ = Some(new.clone()),
            None => self.head = Some(new.clone())
        }

        self.tail = Some(new);
        self.count += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::LinkedList;

    #[test]
    fn create_linked_list() {
        let mut ll = LinkedList::new();
        ll.add_cell(1);
        assert_eq!(1, ll.count)
    }
}