use std::{rc::Rc, cell::RefCell};

#[derive(Clone)]
pub struct Cell<T> {
    succ: Link<T>,
    value: T,
}

type Link<T> = Option<Rc<RefCell<Cell<T>>>>;

impl<T> Cell<T> {
    fn new(value: T) -> Rc<RefCell<Cell<T>>> {
        Rc::new(RefCell::new(Cell {
            succ: None,
            value,
        }))
    }
}

#[derive(Clone)]
pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    count: i32
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
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
    use std::borrow::BorrowMut;

    use crate::linked_list::{LinkedList, Cell, Link};

    #[test]
    fn create_linked_list() {
        let mut ll = LinkedList::new();
        ll.add_cell(1);
        assert_eq!(1, ll.count)
    }

    #[test]
    fn add_multiple_values() {
        let mut ll = LinkedList::new();
        for n in 1..=100 {
            ll.add_cell(n)
        }
        // let _ = (1..=100).into_iter().map(|n| ll.add_cell(n));
        assert_eq!(100, ll.count)
    }

    #[test]
    fn display_multiple_values() {
        let mut ll = LinkedList::new();
        for n in 1..=100 {
            ll.add_cell(n)
        }

        let mut stack: Vec<Link<i32>> = Vec::new();
        stack.push(ll.head);

        while let Some(link) = stack.pop() {
            if let Some(l) = link {
                let cell = (*l).clone().into_inner();
                stack.push(cell.succ);
                println!("Cell value: {}", cell.value);
            }  
        }

        assert_eq!(100, ll.count)
    }
}