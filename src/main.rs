#[macro_use]
extern crate tera;
#[macro_use]
extern crate lazy_static;
//use actix_web::http::{header, Method};
use actix_web::middleware::session;
use actix_web::{fs, middleware, pred, server::HttpServer, App, HttpResponse};
use env_logger;
use futures;
use listenfd::ListenFd;
mod controllers;
pub mod router;
mod run_server;

fn main() {
    //init logger
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    // env_logger::init();

    //init autoreload additional sockets
    let mut listenfd = ListenFd::from_env();
    let mut server_1: run_server::Server = run_server::Server::new("server_1");

    server_1.run();
}
