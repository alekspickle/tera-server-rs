extern crate actix_web;
extern crate listenfd;

use actix_web::{server, App, HttpRequest, Responder};
use listenfd::ListenFd;
// use router::handleRoute;

fn greet(req: &HttpRequest) -> impl Responder {
    let a = req.uri();
    println!("Whoa! uri {:?} request {:?} ", a, req.request());
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::new()
            .resource("/", |r| r.f(greet))
            .resource("/{name}", |r| r.f(greet))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:3000").unwrap()
    };

    server.run();
}

// systemfd --no-pid -s http::3000 -- cargo watch -x run
