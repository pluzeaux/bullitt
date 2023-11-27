use bullitt::add_two_numbers::{add_two_numbers, create_list};

fn main() {
    // let l1 = vec![9,9,9,9,9,9,9];
    // let l2 = vec![9,9,9,9];

    let l1 = vec![2, 4, 9];
    let l2 = vec![5, 6, 4, 9];

    let ln1 = create_list(Vec::from(l1), None);
    let ln2 = create_list(Vec::from(l2), None);

    let mut res = add_two_numbers(ln1, ln2);

    while let Some(i) = res {
        println!("{:?}", i.val);
        res = i.next;
    }
}
