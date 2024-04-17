use std::env;

mod fibonacci;
mod look_and_say;
mod valid_parentheses;
mod collatz;
mod tcp_chat_server;
mod tcp_chat_client;
mod tcp_chat_utils;

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
    } else if run == "tcp_chat_server" {
        tcp_chat_server::test();
    } else if run == "tcp_chat_client" {
        tcp_chat_client::test();
    }
}
