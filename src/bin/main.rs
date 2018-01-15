extern crate cautious_eureka;
use cautious_eureka::thread_pool::ThreadPool;
use cautious_eureka::router;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}


fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let (status_line, contents) = router::router(&buffer);
    let response = format!("{}{}", status_line, contents);
    println!("{}", response);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
