use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use std::{net, thread};

use crate::tcp_chat_utils::{read_message, send_message};

// fn send_loop(stream: Arc<Mutex<net::TcpStream>>) {
//     for i in 0..10000 {
//         {
//             let mut stream = stream.lock().unwrap();
//             send_msg_str(&mut *stream, &format!("Hello, world ({})", i)).unwrap();
//         }

//         sleep(Duration::from_millis(1000));
//     }
// }

fn handle_stream(stream: net::TcpStream, message_q: Arc<Mutex<Vec<String>>>) {
    stream
        .set_read_timeout(Some(Duration::from_millis(100)))
        .unwrap();

    println!("New client connected: {:?}", stream.peer_addr().unwrap());

    let stream = Arc::new(Mutex::new(stream));
    let mut message_q_idx = 0;

    loop {
        {
            let mut stream = stream.lock().unwrap();

            match read_message(&mut *stream) {
                Ok(msg_str) => {
                    if msg_str.len() > 1 {
                        (*message_q.lock().unwrap()).push(msg_str);
                    }
                }
                Err(m_err) => println!("An error occurred while receiving a msg_str: {:?}", m_err),
            };

            message_q.clear_poison();
            let message_q = message_q.lock().unwrap();
            let message_q_size = message_q.len();
            for idx in message_q_idx..message_q_size {
                let message = &(*message_q)[idx];
                send_message(&mut stream, message).unwrap();
            }
            message_q_idx = message_q_size;
        }

        sleep(Duration::from_millis(10));
    }
}

pub fn test() {
    let listener = net::TcpListener::bind("localhost:42069").unwrap();
    let message_q = Arc::new(Mutex::new(Vec::<String>::new()));

    for stream in listener.incoming() {
        let message_q = Arc::clone(&message_q);
        thread::spawn(|| handle_stream(stream.unwrap(), message_q));
    }
}
