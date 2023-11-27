use std::collections::HashMap;

pub fn frequency(s: &str, res: &mut HashMap<String, u32>) {
    s.chars().filter(|c| c.is_alphabetic()).for_each(|c| {
        let k = c.to_uppercase().to_string();
        if res.contains_key(&k) {
            *res.get_mut(&k).unwrap() += 1
        } else {
            res.insert(k.clone(), 1);
        }
    });
}
