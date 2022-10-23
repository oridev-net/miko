use std::io::prelude::*;
use std::net::TcpStream;

pub fn handle_service(headers: &Vec<String>) -> String {
	println!("handling service");
	
	let mut stream = TcpStream::connect("oridev.net:80").unwrap();
	// stream.write(b"");
	for header in headers {
		if header != &headers[1] {
			stream.write(header.as_bytes());
		}
		else {
			stream.write(b"Host: oridev.net");
		}
		println!("{header}");
	}
	stream.flush();

	let mut buf = String::from("");
	stream.read_to_string(&mut buf);
	// println!("{buf}");
	buf
}