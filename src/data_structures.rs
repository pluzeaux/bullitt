use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
struct Node {
    value: String,
    next: Link,
}

type Link = Option<Rc<RefCell<Node>>>;

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

#[derive(Clone)]
pub struct TransactionLog {
    head: Link,
    tail: Link,
    pub length: u64,
}

impl Default for TransactionLog {
    fn default() -> Self {
        Self::new()
    }
}

impl TransactionLog {
    pub fn new() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: String) {
        let new = Node::new(value);

        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Somethimg is terribly wrong")
                .into_inner()
                .value
        })
    }
}

#[cfg(test)]
mod tests {
    // use crate::linked_lists;
    use rand::{thread_rng, Rng};
    use std::collections::LinkedList;
    use test::Bencher;

    use crate::data_structures::TransactionLog;

    const LIST_ITEMS: u64 = 15_000;

    #[bench]
    fn bench_std_linked_list_find(b: &mut Bencher) {
        let mut list = LinkedList::new();
        let items: Vec<String> = (0..LIST_ITEMS)
            .map(|i| format!("INSERT INTO mytable VALUES ({})", i).to_owned())
            .collect();
        for item in items.iter() {
            list.push_back(item.clone());
        }
        let mut rng = thread_rng();

        b.iter(|| {
            let r = rng.gen_range::<usize>(0, LIST_ITEMS as usize);
            list.iter().find(|&x| x == &items[r]).expect("NOT FOUND")
        });
    }

    #[bench]
    fn bench_vec_find(b: &mut Bencher) {
        let mut list = vec![];

        for i in 0..LIST_ITEMS {
            list.push((i, format!("INSERT INTO mytable VALUES ({})", i).to_owned()));
        }
        let mut rng = thread_rng();

        b.iter(|| {
            let r = rng.gen_range::<u64>(0, LIST_ITEMS);
            list.iter().find(|&x| x.0 == r).expect("NOT FOUND")
        });
    }

    #[bench]
    fn bench_linked_list_append(b: &mut Bencher) {
        let mut list = LinkedList::new();
        let mut rng = thread_rng();

        b.iter(|| list.push_back(rng.gen::<u64>()));
    }

    #[bench]
    fn bench_vec_append(b: &mut Bencher) {
        let mut list = vec![];
        let mut rng = thread_rng();

        b.iter(|| list.push(rng.gen::<u64>()));
    }

    #[test]
    fn transaction_log_append() {
        let mut transaction_log = TransactionLog::new();
        assert_eq!(transaction_log.length, 0);
        transaction_log.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        transaction_log.append("INSERT INTO mytable VALUES (2,3,4)".to_owned());
        transaction_log.append("INSERT INTO mytable VALUES (3,4,5)".to_owned());
        assert_eq!(transaction_log.length, 3);
        assert_eq!(
            transaction_log.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(
            transaction_log.pop(),
            Some("INSERT INTO mytable VALUES (2,3,4)".to_owned())
        );
        assert_eq!(
            transaction_log.pop(),
            Some("INSERT INTO mytable VALUES (3,4,5)".to_owned())
        );
        assert_eq!(transaction_log.pop(), None);
    }

    #[test]
    fn transaction_log_pop() {
        let mut list = TransactionLog::new();
        assert_eq!(list.pop(), None);
        list.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        list.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        list.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        assert_eq!(
            list.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(
            list.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(
            list.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(list.pop(), None);
    }
}
