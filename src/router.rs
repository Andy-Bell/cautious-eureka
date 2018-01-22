extern crate httparse;

use std::io::prelude::*;
use std::fs::File;
use std::string::String;
use self::httparse::Request;

pub struct Route<'a>{
    path: String,
    function: &'a Fn(&Request) -> Response_Object,
}

impl<'a> Route<'a> {
    pub fn new<F>(func: &'a F, path: String) -> Route<'a> where F: Fn(&Request) -> Response_Object {
        Route {
            path: path,
            function: func
        }
    }

    pub fn call(&self, req: &Request) -> Response_Object {
        return (self.function)(&req);
    }
}

pub struct Response_Object {
    header: String,
    body: String,
}

impl Response_Object {
    pub fn new(header: String, body: String) -> Response_Object {
        Response_Object {
            header: header,
            body: body,
        }
    }
}

pub struct Router<'b>{
    pub routes: [Route<'b>; 512],
}

impl<'b> Router<'b>{
    pub fn new(routes: [Route; 512]) -> Router {
        Router {
            routes: routes,
        }
    }

    pub fn match_routes(&self, req: Request) -> Response_Object {
        match req.path {
            Some(ref path) => {
                let mut res = Response_Object::new(String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n"), String::from("404.html"));
                for route in self.routes.iter() {
                    if path.to_string() == route.path {
                        res = route.call(&req);
                        return res
                    }
                }
                return res;
            },
            None => {
                panic!("no req.path");
            },
        };
    }

}

pub fn router (buffer: &[u8]) -> (String, String) {
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = Request::new(&mut headers);

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
