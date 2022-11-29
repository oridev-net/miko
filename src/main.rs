use std::{
    fs::File,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

mod service;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let request_line = &http_request[0];

	println!("{http_request:#?}");
 //    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
 //        ("HTTP/1.1 200 OK", "www/index.html")
 //    } else if request_line == "GET /resource/bg1.jpg HTTP/1.1" {
 //        ("HTTP/1.1 200 OK", "www/resources/bg1.jpg")
 //    } else if request_line == "GET /resource/bg2.jpg HTTP/1.1" {
 //        ("HTTP/1.1 200 OK", "www/resources/bg2.jpg")
 //    } else if request_line == "GET /resource/bg3.jpg HTTP/1.1" {
 //        ("HTTP/1.1 200 OK", "www/resources/bg3.jpg")
 //    } else if request_line == "GET /resource/bg4.jpg HTTP/1.1" {
 //        ("HTTP/1.1 200 OK", "www/resources/bg4.jpg")
	// } else if is_requesting_service(request_line) {
 //        ("HTTP/1.1 200 OK", "www/index.html")
 //    } else {
 //        ("HTTP/1.1 404 NOT FOUND", "www/index.html")
 //    };

 //    let mut file = File::open(filename).unwrap();

 //    stream
 //        .write_all(format!("{}\r\n\r\n", status_line).as_bytes())
 //        .unwrap();
 //    let mut buf = [0; 4096];
 //    loop {
 //        let n = file.read(&mut buf).unwrap();

 //        if n == 0 {
 //            break;
 //        }

 //        stream.write_all(&buf[..n]).unwrap();
 //    }
}

// fn is_requesting_service(request_line: &str) -> bool {
// 	let path_started = false;
// 	for i in request_line.chars() {
// 		if i == ' ' {
// 			let path_started = true;
// 		}
// 	}
// 	false
// }