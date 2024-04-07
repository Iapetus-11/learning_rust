use std::env;

mod fibonacci;
mod look_and_say;

fn main() {
    let run = env::args().nth(1).expect("Specify the name of the file to run the test function of");

    if run == "fibonacci" {
        fibonacci::test();
    } else if run == "look_and_say" {
        look_and_say::test();
    }
}
