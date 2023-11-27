use bullitt::longest_substring::length_of_longest_substring;
use std::time::{Duration, Instant};

fn main() {
    let start: Instant = Instant::now();

    println!("{}", length_of_longest_substring(String::from("abcabcbb")));
    println!("{}", length_of_longest_substring(String::from("bbbbb")));
    println!("{}", length_of_longest_substring(String::from("pwwkew")));
    println!("{}", length_of_longest_substring(String::from(" ")));
    println!("{}", length_of_longest_substring(String::from("au")));
    println!("{}", length_of_longest_substring(String::from("nfpdmpi")));
    println!("{}", length_of_longest_substring(String::from("nfpdmpieg")));

    let duration: Duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
