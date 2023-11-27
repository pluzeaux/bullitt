use bullitt::luhn::is_valid;

fn main() {
    let s = "8273 1232 7352 0569"; //false
    println!("String: {} is_valid: {:?}", s, is_valid(s));

    let s = "055 444 285"; //true
    println!("String: {} is_valid: {:?}", s, is_valid(s));

    let s = "055 444 286"; //false
    println!("String: {} is_valid: {:?}", s, is_valid(s));

    let s = "095 245 88"; //true
    println!("String: {} is_valid: {:?}", s, is_valid(s));

    let s = "055a 444 285"; //false
    println!("String: {} is_valid: {:?}", s, is_valid(s));

    let s = "055-444-285"; //false
    println!("String: {} is_valid: {:?}", s, is_valid(s));

    let s = "1249①"; //false
    println!("String: {} is_valid: {:?}", s, is_valid(s));
}
