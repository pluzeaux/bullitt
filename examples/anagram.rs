use bullitt::anagram::process_anagram_case;

fn main() {
    // let word = "ΑΒΓ";
    // let inputs = ["ΑΒγ"];

    let word = "banana";
    let inputs = ["bAnana"];

    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}
