// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    ln1: Option<Box<ListNode>>,
    ln2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let l1: Vec<i32> = listnode_to_vec(ln1);
    let l2: Vec<i32> = listnode_to_vec(ln2);

    let len1 = l1.len();
    let len2 = l2.len();

    let max_len = if len1 > len2 { len1 } else { len2 };

    let l1 = if len1 < max_len {
        fill_vec(l1, max_len as i32)
    } else {
        l1
    };
    let l2 = if len2 < max_len {
        fill_vec(l2, max_len as i32)
    } else {
        l2
    };

    let mut res: Vec<i32> = Vec::new();
    let mut retenue: i32 = 0;

    for i in 0..max_len {
        let s = l1[i] + l2[i] + retenue;
        retenue = 0;
        if s > 9 {
            res.push(s % 10);
            retenue += 1;
        } else {
            res.push(s)
        }
    }

    if retenue > 0 {
        res.push(retenue)
    }

    create_list(res, None)
}

fn listnode_to_vec(ln: Option<Box<ListNode>>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    let mut l = ln;

    while let Some(lt) = l {
        res.push(lt.val);
        l = lt.next;
    }

    res
}

fn fill_vec(mut v1: Vec<i32>, i: i32) -> Vec<i32> {
    // let diff = i.abs_diff(v1.len() as i32);
    let diff = (i - v1.len() as i32).abs();
    for _ in 0..diff {
        v1.push(0_i32)
    }

    v1
}

pub fn create_list(mut l: Vec<i32>, ln: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if let Some(v) = l.pop() {
        return create_list(l, Some(Box::new(ListNode { val: v, next: ln })));
    }

    ln
}
