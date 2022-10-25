use std::io::prelude::*;
use std::net::TcpStream;
use std::io::{Error, ErrorKind};
use std::fs;

pub fn handle_service(headers: &Vec<String>) -> std::io::Result<String> {
	let mut log: String = String::from("");
	println!("handling service");

	let mut stream = TcpStream::connect("reddit.com:80")?;

    handle_headers(&stream, headers)?;
	log = format!("{log}\n{headers:#?}");

    stream.write(b"GET / HTTP/1.0\r\nHost: reddit.com\r\n\r\n")?;
	log = format!("{log}\nGET / HTTP/1.0\r\nHost: reddit.com\r\n\r\n");

    stream.flush()?;

    let mut buf;
	let mut requested_service: String = String::from("");
    loop {
        buf = [0; 512];
        let result = stream.read(&mut buf);
        match result {
            Ok(n) => {
                // println!("Received {} bytes",n);
				requested_service.push_str(std::str::from_utf8(&buf).unwrap());
                if n == 0 {
					// println!("{requested_service}");
					log = format!("{log}\n{requested_service}");
                    fs::write("test/log", log)?;
					return Ok(requested_service);
                }
            },
            _ => {return Err(Error::new(ErrorKind::Other, "yo no connecto"));},
        }
    }
}

fn handle_headers(stream: &TcpStream, headers: &Vec<String>) -> std::io::Result<()> {

    println!("{headers:#?}");

    Ok(())
}