use std::io::{self};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use std::{net, thread};

use crate::tcp_chat_utils::{read_message, send_message};

fn take_input() -> Result<String, io::Error> {
    let mut out = String::new();
    io::stdin().read_line(&mut out)?;
    return Ok(out);
}

fn recv_loop(stream: Arc<Mutex<net::TcpStream>>) {
    loop {
        {
            let mut s = stream.lock().unwrap();
            match read_message(&mut *s) {
                Ok(m_str) => {
                    if m_str.len() > 1 {
                        println!("<- {}", m_str);
                    }
                }
                Err(m_err) => println!("An error occurred while receiving a message: {:?}", m_err),
            };
        }

        sleep(Duration::from_millis(100));
    }
}

pub fn test() {
    let stream = net::TcpStream::connect("localhost:42069").unwrap();
    stream
        .set_read_timeout(Some(Duration::from_millis(100)))
        .unwrap();

    let stream = Arc::new(Mutex::new(stream));

    {
        let cloned_stream = Arc::clone(&stream);
        thread::spawn(|| recv_loop(cloned_stream));
    }

    loop {
        let mut input = take_input().unwrap();
        input = String::from(input.trim_end());

        if input.len() >= 1 {
            let mut s = stream.lock().unwrap();
            send_message(&mut *s, &input).unwrap();
        }
    }
}
