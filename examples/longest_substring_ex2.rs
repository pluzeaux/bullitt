use bullitt::longest_substring::length_of_longest_substring_init;
use std::time::{Duration, Instant};

fn main() {
    let start: Instant = Instant::now();

    println!(
        "{}",
        length_of_longest_substring_init(String::from("abcabcbb"))
    );
    println!(
        "{}",
        length_of_longest_substring_init(String::from("bbbbb"))
    );
    println!(
        "{}",
        length_of_longest_substring_init(String::from("pwwkew"))
    );
    println!("{}", length_of_longest_substring_init(String::from(" ")));
    println!("{}", length_of_longest_substring_init(String::from("au")));
    println!(
        "{}",
        length_of_longest_substring_init(String::from("nfpdmpi"))
    );
    println!(
        "{}",
        length_of_longest_substring_init(String::from("nfpdmpieg"))
    );

    let duration: Duration = start.elapsed();
    println!(
        "Time elapsed in cartesian_product function is: {:?}",
        duration
    );
}
