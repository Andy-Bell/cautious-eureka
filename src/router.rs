use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
use std::fs::File;
use std::string::String;

pub fn router (mut stream: &TcpStream) -> (String, String) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let s = match str::from_utf8(&buffer) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let array: Vec<&str> = s.split(" ").collect();

    let (status_line, filename) = if array[0] == "GET" && array[1] == "/" {
        (String::from("HTTP/1.1 200 OK\r\n\r\n"), "hello.html")
    } else {
        (String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n"), "404.html")
    };

    let mut file = File::open(format!("views/{}", filename)).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    return (status_line, contents)
}
