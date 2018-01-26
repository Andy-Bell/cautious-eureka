extern crate httparse;
extern crate cautious_eureka;
use cautious_eureka::thread_pool::ThreadPool;
use cautious_eureka::router;
use self::httparse::Request;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
// Commented out due to line 21 no longer needing it
// use std::sync::Arc;
// use std::sync::Mutex;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);
    // Commented out until I can work out why this is not working for 
    // threading issue, temporary fix on line 24 in just using a new
    // router per request
    // 
    // let server_router = Arc::new(Mutex::new(router_config()));

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream, router_config());
        });
    }

    println!("Shutting down.");
}

fn router_config<'c>() -> router::Router<'c> {
    let mut routes: Vec<router::Route> = Vec::new();

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = Request::new(&mut headers);
    let index = make_route(
            String::from("/"),
            &index(&req)
        );
    routes.push(
        index
    );

    return router::Router::new(routes);
}


fn handle_connection(mut stream: TcpStream,
                     server_router: router::Router ) {

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

fn make_route<'a, F>(route: String, func:&'a F) -> router::Route<'a>
    where
    F: Fn(&Request) -> router::ResponseObject{
        return router::Route::new(route, func);
    }

fn index(request: &Request) -> router::ResponseObject {
    return router::ResponseObject::new(String::from("HTTP/1.1 200 OK\r\n\r\n"), String::from("hello.html"));
}
