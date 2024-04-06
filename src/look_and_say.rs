#![allow(dead_code)]

pub fn test() {
    let mut last = String::from("1");

    for _ in 0..10 {
        last = look_and_say(&last);
        println!("{}", last);
    }
}

fn look_and_say(n_str: &str) -> String {
    let mut out = String::new();

    let mut current_char: char = n_str.chars().nth(0).unwrap();
    let mut current_count = 0;
    
    for c in n_str.chars().chain(" ".chars()) {
        if current_char != c {
            out.push_str(&current_count.to_string());
            out.push(current_char);

            current_char = c;
            current_count = 0;
        }

        current_count += 1;
    }

    return out;
}
