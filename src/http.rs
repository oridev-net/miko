use std::io::{prelude::*, BufReader};
use std::net::TcpStream;

pub struct Header {
	pub name: String,
	pub value: String
}

pub struct Query {
	pub id: String,
	pub value: String
}

pub struct Path {
	pub dirs: Vec<String>,
	pub query: Vec<Query>
}

// impl Path {
// 	fn parse(path: String) {
// 		dir: Vec
// 	}
// }

pub struct RequestLine {
	pub http_method: String,
	pub path: String,
	pub http_version: String
}

pub struct StatusLine {
	pub http_status: String,
	pub path: String,
	pub http_version: String
}

pub struct Request {
	pub request_line: RequestLine,
	pub headers: Vec<Header>,
}

impl Request {
	pub fn parse(unparsed_request: BufReader<&mut TcpStream>) -> Request {
		let mut parsed: Vec<_> = unparsed_request
        	.lines()
	        .map(|result| result.unwrap())
	        .take_while(|line| !line.is_empty())
	        .collect();
		// println!("{}", parsed[0]);
		let mut http_method: String = "".to_string();
		let mut path: String = "".to_string();
		let mut http_version: String = "".to_string();
		let mut section: u8 = 0;
		for letter in parsed[0].chars() {
			if letter == ' ' {
				section+=1;
			}
			else {
				if section == 0 {
					http_method = http_method+&letter.to_string();
				}
				if section == 1 {
					path = path+&letter.to_string();
				}
				if section == 2 {
					http_version = http_version+&letter.to_string();
				}
			}
		}
		parsed.remove(0);
		let mut headers: Vec<Header> = vec![];
		for header in parsed {
			// println!("{header}");
			let mut value_started1 = false;
			let mut value_started2 = false;
			let mut name = "".to_string();
			let mut value = "".to_string();
			for (_i, letter) in header.chars().enumerate() {
				if letter == ':' {
					value_started1 = true;
				}
				else if letter == ' '&&value_started1 {
					value_started2 = true;
				}
				else if value_started2 {
					value = value + &letter.to_string();
				}
				else {
					name = name + &letter.to_string();
				}
			}
			name=name.replace(" ","");
			headers.push(Header {
				name: name,
				value: value
			});
		}
		Request {
			request_line: RequestLine {
				http_method: http_method,
				path: path,
				http_version: http_version
			},
			headers: headers,
		}
	}
}

pub struct Response {
	pub unparsed_response: BufReader<TcpStream>,
	pub status_line: RequestLine,
	pub headers: Vec<Header>,
	pub body: String
}