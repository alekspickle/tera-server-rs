#[macro_use]
extern crate tera;
#[macro_use]
extern crate lazy_static;
use futures;
use env_logger;
use actix_web::http::{header, Method};
use actix_web::middleware::session;
use actix_web::{fs, middleware, pred, server, App, HttpResponse};
use listenfd::ListenFd;
mod router;
mod controllers;
// mod server::{Server};

fn main() {
    //init logger
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    // env_logger::init();

    //init autoreload additional sockets
    let mut listenfd = ListenFd::from_env();

    let mut server = server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .middleware(session::SessionStorage::new(
                session::CookieSessionBackend::signed(&[0; 32]).secure(false),
            ))
            .handler("/static", fs::StaticFiles::new("static").unwrap())
            .resource("/favicon", |r| r.get().f(router::favicon))
            .resource("/", |r| r.get().f(router::index))
            .resource("/triplets", |r| r.get().f(router::triplets))
            .resource("/generate_triplets", |r| r.post().f(router::generate_triplets))
            .resource("/calculate", |r| r.get().f(router::calculate))
            // redirect
            .resource("/test", |r| {
                r.get().f(|req| {
                    println!("{:?}", req);
                    HttpResponse::Found()
                        .header(header::LOCATION, "pages/index.html")
                        .finish()
                })
            })
            // default
            .default_resource(|r| {
                // 404 for GET request
                r.method(Method::GET).f(router::p404);
                // deny all requests that are not `GET`
                r.route()
                    .filter(pred::Not(pred::Get()))
                    .f(|_| HttpResponse::MethodNotAllowed());
            })
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server
            .bind("127.0.0.1:3000")
            .expect("Could not bind to port 3000")
    };

    println!("Server is running on 127.0.0.1:3000");
    server.run()
}
