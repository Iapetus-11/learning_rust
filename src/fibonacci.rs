#![allow(dead_code)]

use std::io::stdin;

pub fn test() {
    let mut s = String::new();

    stdin().read_line(&mut s).expect("A valid string?");

    let n = s.trim_end().parse::<i128>().expect("A valid number");

    for idx in 0..n {
        println!("{}", fibonacci(idx));
    }
}

fn fibonacci(n: i128) -> i128 {
    return match n {
        0 => 1,
        1 => 2,
        _ => fibonacci(n-1) + fibonacci(n-2),
    };
}
