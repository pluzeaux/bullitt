use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut stack = vec![root];
    let mut hm: HashMap<i32, i32> = HashMap::new();

    while let Some(current) = stack.pop().unwrap() {
        let k = current.borrow().val;
        stack.push(current.borrow().left.to_owned());
        stack.push(current.borrow().right.to_owned());

        let v: i32 = if hm.contains_key(&k) {
            hm.get(&k).unwrap() + 1
        } else {
            1
        };

        hm.insert(k, v);
    }

    let max = hm.values().max().unwrap();
    hm.iter()
        .filter(|x| x.1 == max)
        .map(|f| *f.0)
        .collect::<Vec<i32>>()
}
