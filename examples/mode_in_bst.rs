use std::{cell::RefCell, rc::Rc};

use bullitt::mode_in_bst::{find_mode, TreeNode};

fn main() {
    // [1, 3, 6, 6, 6, 6, 7, 7, 12, 12, 17]
    let mut root = TreeNode::new(1);
    let mut n1 = TreeNode::new(3);
    let mut n2 = TreeNode::new(6);
    let mut n3 = TreeNode::new(6);
    let mut n4 = TreeNode::new(6);
    let n5 = TreeNode::new(6);
    let mut n6 = TreeNode::new(7);
    let n7 = TreeNode::new(7);
    let mut n8 = TreeNode::new(12);
    let n9 = TreeNode::new(12);
    let n10 = TreeNode::new(17);
    n8.right = Some(Rc::new(RefCell::new(n10)));
    n8.left = Some(Rc::new(RefCell::new(n9)));
    n6.right = Some(Rc::new(RefCell::new(n8)));
    n6.left = Some(Rc::new(RefCell::new(n7)));
    n4.right = Some(Rc::new(RefCell::new(n6)));
    n4.left = Some(Rc::new(RefCell::new(n5)));
    n3.left = Some(Rc::new(RefCell::new(n4)));
    n2.left = Some(Rc::new(RefCell::new(n3)));
    n1.right = Some(Rc::new(RefCell::new(n2)));
    root.right = Some(Rc::new(RefCell::new(n1)));

    let res = find_mode(Some(Rc::new(RefCell::new(root))));
    for i in res {
        println!("{:?}", i)
    }
}
