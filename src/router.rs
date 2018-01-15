extern crate httparse;

use std::io::prelude::*;
use std::fs::File;
use std::string::String;

pub fn router (buffer: &[u8]) -> (String, String) {
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);

    let _res = req.parse(buffer).unwrap();

    let (status_line, filename) = match req.path {
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
        };


    let mut file = File::open(format!("views/{}", filename)).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    return (status_line, contents)
}
