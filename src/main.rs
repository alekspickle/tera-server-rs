#[macro_use]
extern crate tera;
#[macro_use]
extern crate lazy_static;

mod controllers;
pub mod router;
mod run_server;

// use env_logger;
use run_server::Server;


fn main() {
    //  init logger
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    //  env_logger::init();

    let mut server_1 = Server {
        name: "server_1".to_owned(),
        address: "127.0.0.1".to_owned(),
        port: "3000".to_owned(),
    };

    server_1.start();
}
