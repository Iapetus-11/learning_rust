use std::env;

mod fibonacci;
mod look_and_say;
mod valid_parentheses;
mod collatz;

fn main() {
    let run = env::args().nth(1).expect("Specify the name of the file to run the test function of");

    if run == "fibonacci" {
        fibonacci::test();
    } else if run == "look_and_say" {
        look_and_say::test();
    } else if run == "valid_parentheses" {
        valid_parentheses::test();
    } else if run == "collatz" {
        collatz::test();
    }
}
