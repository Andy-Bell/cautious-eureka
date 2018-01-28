extern crate httparse;

use std::io::prelude::*;
use std::fs::File;
use std::string::String;
use self::httparse::Request;

pub struct Route<'a>{
    path: String,
    function: &'a mut FnMut(&Request) -> ResponseObject,
}

impl<'a> Route<'a> {
    pub fn new<F>(path: String, func: &'a mut F) -> Route<'a> 
        where 
        F: FnMut(&Request) -> ResponseObject{
        Route {
            path: path,
            function: func
        }
    }

    pub fn call(&mut self, req: &Request) -> ResponseObject {
        return (self.function)(&req);
    }
}

pub struct ResponseObject {
    pub header: String,
    pub body: String,
}

impl ResponseObject{
    pub fn new(header: String, body: String) -> ResponseObject {
        ResponseObject {
            header: header,
            body: body,
        }
    }
}

pub struct Router<'b>{
    pub routes: Vec<Route<'b>>,
}

impl<'b> Router<'b>{
    pub fn new(routes: Vec<Route>) -> Router {
        Router {
            routes: routes,
        }
    }

    pub fn match_routes(&self, req: Request) -> ResponseObject {
        match req.path {
            Some(ref path) => {
                let mut res = ResponseObject::new(String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n"), String::from("404.html"));
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
