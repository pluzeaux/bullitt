pub fn is_valid(code: &str) -> bool {
    let s = remove_whitespaces(code);
    if s == "0" || check_input(s.as_str()).is_err() {
        false
    } else {
        calc_luhn(s.as_str()) % 10 == 0
    }
}

fn remove_whitespaces(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect::<String>()
}

fn calc_luhn(s: &str) -> u32 {
    s.chars()
        .rev()
        .enumerate()
        .map(|(i, v)| {
            let vd = v.to_digit(10).unwrap();

            if (i + 1) % 2 == 0 {
                let r = vd * 2;
                if r > 9 {
                    r - 9
                } else {
                    r
                }
            } else {
                vd
            }
        })
        .sum::<u32>()
}

fn check_input(s: &str) -> Result<String, String> {
    let r = s.chars().all(|c| c.is_ascii_digit());
    if r {
        Ok(s.to_string())
    } else {
        Err("String contains a no numeric character".to_string())
    }
}
