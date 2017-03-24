mod http;

use std::fs::File;
use std::io::Read;

use http::BodyType;

fn main() {
    http::start_server(move |request, mut response| {
        println!("Method: {}\tRoute: {}", request.method, request.route);
        let body;
        if request.route == "/favicon.ico" {
            let mut vec = Vec::new();
            let mut file = File::open("favicon.ico").unwrap();
            file.read_to_end(&mut vec).unwrap();
            body = BodyType::Bytes(vec);
        } else {
            let mut file = File::open("hello_world.html").unwrap();
            let mut string = String::new();
            file.read_to_string(&mut string).unwrap();
            body = BodyType::String(string);
        }
        response.set_body(body);
        response.send();
    });
}
