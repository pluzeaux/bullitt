use bullitt::restore_array::restore_array;

fn main() {
    // let input = vec![vec![2,1],vec![3,4],vec![3,2]];
    let input = vec![
        vec![4, -1],
        vec![6, 4],
        vec![8, 6],
        vec![9, 8],
        vec![9, 7],
        vec![7, 5],
        vec![-7, 5],
        vec![-4, -7],
    ];

    let res = restore_array(input);

    println!("{:?}", res);
}
