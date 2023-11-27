use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut res: HashSet<&str> = HashSet::new();

    let w: Vec<char> = word.to_uppercase().chars().collect::<Vec<char>>();
    let mut ws = w.clone();
    ws.sort();

    for i in possible_anagrams {
        let a: Vec<char> = i.to_uppercase().chars().collect();
        let mut ast = a.clone();
        ast.sort();

        if w != a && ws == ast {
            println!("word : {}, *i : {}", word, *i);
            res.insert(i);
        }
    }
    res
}

pub fn process_anagram_case(word: &str, inputs: &[&str], expected: &[&str]) {
    let result = anagrams_for(word, inputs);
    let expected: HashSet<&str> = expected.iter().cloned().collect();
    assert_eq!(result, expected);
}

#[test]
fn does_not_detect_a_differently_cased_word_as_its_own_anagram() {
    let word = "banana";
    let inputs = ["bAnana"];
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
fn does_not_detect_a_differently_cased_unicode_word_as_its_own_anagram() {
    let word = "ΑΒΓ";
    let inputs = ["ΑΒγ"];
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}

#[test]
fn case_insensitive_anagrams() {
    let word = "Orchestra";
    let inputs = ["cashregister", "Carthorse", "radishes"];
    let outputs = vec!["Carthorse"];
    process_anagram_case(word, &inputs, &outputs);
}

#[test]
fn no_matches() {
    let word = "diaper";
    let inputs = ["hello", "world", "zombies", "pants"];
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}

#[test]
fn detect_simple_anagram() {
    let word = "ant";
    let inputs = ["tan", "stand", "at"];
    let outputs = vec!["tan"];
    process_anagram_case(word, &inputs, &outputs);
}
