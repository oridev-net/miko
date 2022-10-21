// use std::io::prelude::*;
// use std::net::TcpStream;
mod server;

fn main() {
	server::start_server();
	// let mut stream = TcpStream::connect("oridev.net:80").unwrap();

 //    stream.write(b"GET / HTTP/1.1\r\n").unwrap();
 //    stream.write(b"GET / HTTP/1.1\r\n").unwrap();
 //    stream.write(b"GET / HTTP/1.1\r\n").unwrap();
 //    stream.write(b"GET / HTTP/1.1\r\n").unwrap();
 //    stream.write(b"GET / HTTP/1.1\r\n").unwrap();
	// println!("wrote");
 //    stream.flush().unwrap();
	// println!("flushed");
	// let mut buffer: String = String::from("");
	// stream.read_to_string(&mut buffer).unwrap();
	// println!("{}", buffer);
}
