extern crate httparse;

use std::io::prelude::*;
use std::fs::File;
use std::string::String;
use self::httparse::Request;

pub struct Route {
    path: String,
    function: Box<Fn(&Request) -> ResponseObject>,
}

impl Route {
    pub fn new<F>(path: String, func: F) -> Route
        where 
        F: Fn(&Request) -> ResponseObject + 'static
    {
        Route {
            path: path,
            function: Box::new(func)
        }
    }

    pub fn call(&self, req: &Request) -> ResponseObject {
        return (self.function)(&req);
    }
}

#[derive(Clone)]
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

pub struct Router {
    pub routes: Vec<Route>,
}

impl Router {
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

