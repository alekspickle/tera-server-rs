//! # This is my web server in attempt to learn Rust.
//! 
//! Rust is famous for it`s brutal learning curve, so I am doing some basics here.
//! in order to break through it.
//!
//! ## For now I have such examples here:
//! - convert temperature Celsius/Fahrenheit both cases given the number
//! - calculate given number of Pythagorean triplets 
//! - calculate sertain Fibonacci number
//! - assemble christmas song procedurally
//! - accept multipart download
//! 
//! And show it in the browser.
//!
//! ## Failed 
//! - added rectangle drawing from rustbook but to be able to display it,
//!     need to figure out how to create DOM node (with < br> f.e.)
//!     because without it, it treated as just innerHTML text
//! 
//! ## Doing now:
//! - find a way to render through tera as an AppData field, without *static ref TEMPLATE*
//! - do some multithreading tasks
//! - do some futures-related tasks

#[macro_use]
extern crate tera;
#[macro_use]
extern crate lazy_static;

pub mod controllers;
pub mod router;
pub mod server;

// use env_logger;
use server::Server;
use std::env;

fn main() {
    //set RUST_LOG enviroment variable to enable logs from actix_web
    env::set_var("RUST_LOG", "actix_web=info");
    //  init logger
    //  env_logger::init();
    let port = env::var("PORT").unwrap_or_else(|_e| "3000".into());

    let mut server_1 = Server {
        name: "server_1".to_owned(),
        address: "127.0.0.1".to_owned(),
        port: port,
    };

    server_1.start();
}
