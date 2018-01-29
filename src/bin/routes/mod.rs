extern crate httparse;
extern crate cautious_eureka;

use cautious_eureka::router;
use self::httparse::Request;

pub fn config() -> router::Router {
    let mut routes: Vec<router::Route> = Vec::new();

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let req = Request::new(&mut headers);
    let index_func = index(&req);
    let index = make_route(
        String::from("/"),
        move |_| index_func.clone()
    );
    routes.push(
        index
    );

    return router::Router::new(routes);
}

fn make_route<F>(route: String, func: F) -> router::Route
    where
    F: Fn(&Request) -> router::ResponseObject + 'static
{
    return router::Route::new(route, func);
}

fn index(_request: &Request) -> router::ResponseObject {
    return router::ResponseObject::new(String::from("HTTP/1.1 200 OK\r\n\r\n"), String::from("hello.html"));
}
