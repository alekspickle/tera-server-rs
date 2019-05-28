//! # This is my web server in attempt to learn Rust.
//! 
//! Rust is famous for it`s brutal learning curve, so I am doing some basics here.
//! in order to break through it.
//!
//!
//!
//! ## For now I have such examples here:
//! - convert temperature Celsius/Fahrenheit both cases given the number
//! - calculate given number of Pythagorean triplets 
//! - calculate sertain Fibonacci number
//! - assemble christmas song procedurally
//! 
//! And show it in the browser.
//! 
//! ## Doing now:
//! - find a way render through tera without *static ref*
//! - process multipart image upload requests
//! - add rectangle route and look how it will display
//! - do some multithreading tasks
//! - do some futures-related tasks

#[macro_use]
extern crate tera;
#[macro_use]
extern crate lazy_static;

pub mod controllers;
pub mod router;
pub mod run_server;

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
