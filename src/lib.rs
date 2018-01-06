use std::io::prelude::*;
use std::net::TcpStream;
use std::fs::File;

pub mod thread_pool;
pub mod worker;

pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

pub enum Message {
    NewJob(Job),
    Terminate,
}

pub fn router (mut stream: &TcpStream) -> (std::string::String, std::string::String) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        (String::from("HTTP/1.1 200 OK\r\n\r\n"), "views/hello.html")
    } else {
        (String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n"), "views/404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    return (status_line, contents)
    
}
