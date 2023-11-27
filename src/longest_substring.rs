pub fn length_of_longest_substring_init(s: String) -> i32 {
    let mut b = 0;
    let mut st1: &str = "";
    let mut st2: &str = "";

    if s.len() == 1 {
        return 1;
    }

    for i in 1..s.len() {
        dbg!(i);
        dbg!(st1);
        let c = s.chars().nth(i).unwrap();
        dbg!(c);
        if s[b..i].contains(c) {
            dbg!(&s[b..i]);
            if s[b..i].len() > st1.len() {
                st1 = &s[b..i]
            }
            b = s[b..i].find(c).unwrap() + b + 1;
        } else {
            st2 = &s[b..=i];
            dbg!(st2);
        }
    }

    if st1.len() > st2.len() {
        st1.len() as i32
    } else {
        st2.len() as i32
    }
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max_len: usize = 0;

    // [1] longest substring is the one with the largest
    //     difference of positions of repeated characters;
    //     thus, we should create a storage for such positions
    let mut pos: [usize; 128] = [0; 128];

    // [2] while iterating through the string (i.e., moving
    //     the end of the sliding window), we should also
    //     update the start of the window
    let mut start: usize = 0;

    for (end, ch) in s.chars().enumerate() {
        // [3] get the position for the start of sliding window
        //     with no other occurences of 'ch' in it
        start = start.max(pos[ch as usize]);

        // [4] update maximum length
        max_len = max_len.max(end - start + 1);

        // [5] set the position to be used in [3] on next iterations
        pos[ch as usize] = end + 1;
    }

    max_len as i32
}
