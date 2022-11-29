use std::io::BufReader;

pub struct Header {
	name: String,
	value: String
}

pub struct RequestLine {
	http_method: String,
	path: String,
	http_version: String
}

pub struct StatusLine {
	http_method: String,
	path: String,
	http_version: String
}

pub struct Request {
	unparsed_request: BufReader,
	request_line: RequestLine,
	headers: Vec<Header>,
	body: String
}

pub struct Response {
	unparsed_request: BufReader,
	status_line: RequestLine,
	headers: Vec<Header>,
	body: String
}