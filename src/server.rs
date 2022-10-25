use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    env
};

#[path = "./service.rs"]
mod service;

pub fn start_server() {
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

	let requested_service = service::handle_service(&http_request).unwrap();

	let response = format!("{requested_service}");

    stream.write_all(response.as_bytes()).unwrap();
}