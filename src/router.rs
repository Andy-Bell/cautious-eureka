extern crate httparse;

use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
use std::fs::File;
use std::string::String;

pub fn router (mut stream: &TcpStream) -> (String, String) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);

    let res = req.parse(&buffer).unwrap();

    let (status_line, filename) = if res.is_partial() {
        match req.path {
            Some(ref path) => {
                if path.to_string() == "/" {
                    (String::from("HTTP/1.1 200 OK\r\n\r\n"), "hello.html")
                } else {
                    (String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n"), "404.html")
                }
            },
            None => {
                panic!("no req.path");
            }
        }
    };

    let mut file = File::open(format!("views/{}", filename)).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    return (status_line, contents)
}
