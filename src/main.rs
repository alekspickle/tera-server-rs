#[macro_use]
extern crate tera;
#[macro_use]
extern crate lazy_static;
use actix_web::http::{header, Method};
use actix_web::middleware::session;
use actix_web::{fs, middleware, pred, server::HttpServer, App, HttpResponse};
use env_logger;
use futures;
use listenfd::ListenFd;
mod controllers;
mod router;
mod run_server;

fn main() {
    //init logger
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    // env_logger::init();

    //init autoreload additional sockets
    let mut listenfd = ListenFd::from_env();

    let mut server_1 = HttpServer::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .middleware(session::SessionStorage::new(
                session::CookieSessionBackend::signed(&[0; 32]).secure(false),
            ))
            .handler("/static", fs::StaticFiles::new("static").unwrap())
            .resource("/favicon", |r| r.get().f(router::favicon))
            .resource("/", |r| r.get().f(router::index))
            .resource("/triplets", |r| r.get().f(router::triplets))
            .resource("/generate_triplets", |r| {
                r.post().f(router::generate_triplets)
            })
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

    server_1 = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server_1.listen(l)
    } else {
        server_1
            .bind("127.0.0.1:3000")
            .expect("Could not bind to port 3000")
    };

    server_1.run();
    println!("Server is running on 127.0.0.1:3000");
}
