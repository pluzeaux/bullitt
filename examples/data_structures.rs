use bullitt::data_structures::TransactionLog;

fn main() {
    let mut tl = TransactionLog::new();
    tl.append(String::from("1"));
    tl.append(String::from("2"));
    tl.append(String::from("3"));
    tl.append(String::from("4"));
    tl.append(String::from("5"));
    tl.append(String::from("6"));
    tl.append(String::from("7"));
    tl.append(String::from("8"));
    tl.append(String::from("9"));
    tl.append(String::from("10"));
    tl.append(String::from("11"));
    let _s1 = tl.pop();
    let _s2 = tl.pop();
}
