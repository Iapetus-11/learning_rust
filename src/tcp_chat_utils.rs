use std::io;
use std::io::{Read, Write};
use std::net;

pub fn read_message(stream: &mut net::TcpStream) -> Result<String, io::Error> {
    let mut size_buf = [0_u8; (usize::BITS / 8) as usize];

    if stream.read(&mut size_buf).unwrap_or(0) == 0 {
        return Ok("".to_string());
    }

    let data_size = usize::from_be_bytes(size_buf);

    let mut data_buf = vec![0_u8; data_size];
    stream.read_exact(&mut data_buf)?;

    return Ok(String::from_utf8(data_buf).unwrap());
}

pub fn send_message(stream: &mut net::TcpStream, message: &str) -> Result<(), io::Error> {
    stream.write(&message.len().to_be_bytes())?;
    stream.write(&message.as_bytes())?;
    stream.flush()?;

    return Ok(());
}
