use std::net::TcpStream;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

pub struct Request {
    pub body : String,
    pub method : String,
    pub route : String,
    pub headers : HashMap<String, String>
}

impl Request {
    pub fn parse_request(stream : & TcpStream) -> Request{
        let mut reader = BufReader::new(stream);
        let mut body = String::new();
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        body.push_str(&line);
        let mut iter = line.split(' ');
        let method = String::from(iter.next().unwrap());
        let route = String::from(iter.next().unwrap());
        let mut headers_str = String::new();
        let mut line = String::from("asfd");
        while !line.trim().is_empty() {
            line.clear();
            reader.read_line(&mut line).unwrap();
            body.push_str(&line);
            headers_str.push_str(&line);
        }

        let mut headers = HashMap::new();
        for line in headers_str.lines().filter(|line| !line.is_empty()) {
            let mut iter = line.split(':');
            headers.insert(iter.next().unwrap().to_string(), iter.next().unwrap().to_string());
        }

        Request{body: body, method: method, headers: headers, route: route}
    }
}
