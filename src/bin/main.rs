extern crate httparse;
extern crate cautious_eureka;
use cautious_eureka::thread_pool::ThreadPool;
use cautious_eureka::router;
use self::httparse::Request;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);
    let server_router = RouterConfig();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream, &server_router);
        });
    }

    println!("Shutting down.");
}

fn RouterConfig<'c>() -> router::Router<'c> {
    let mut routes: Vec<router::Route> = Vec::new();

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = Request::new(&mut headers);
    routes.push(router::Route::new(String::from("/"), &index(&req)));

    return router::Router::new(routes);
}


fn handle_connection(mut stream: TcpStream, server_router: &router::Router ) {

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = Request::new(&mut headers);

    let _res = req.parse(&buffer).unwrap();

    let response_object = server_router.match_routes(req);

    //let (status_line, contents) = router(&buffer);
    //let response = format!("{}{}", status_line, contents);
    //println!("{}", response);

    let response = format!("{}{}", response_object.header, response_object.body);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn index(request: &Request) -> router::ResponseObject {
    return router::ResponseObject::new(String::from("HTTP/1.1 200 OK\r\n\r\n"), String::from("hello.html"));
}
