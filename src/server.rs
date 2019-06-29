//! ## Server
//! Separate server setup for easier multi-server setup.
//!
//!TODO: measure resourses with multiple instances running
//! with *debug* and *release* builds.
//!
//!
//!

// use crate::controllers::AppData;
use crate::router;

use std::cell::Cell;

use actix_files as fs;
use actix_web::http::header;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};

use web::{get, post, resource, route};

///Server struct for each server to create
pub struct Server {
    pub name: String,
    pub address: String,
    pub port: String,
}

impl Server {
    ///start function:
    /// - define app data
    /// - define routes
    /// - bind domain and port to server instance
    /// - run the server
    pub fn start(&mut self) {
        let path = self.address.to_owned() + ":" + &self.port;

        let server = HttpServer::new(|| {
            let mut tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*"));
            tera.autoescape_on(vec![".css", ".svg", ".html", ".sql"]);

            App::new()
                .data(tera)
                .data(Cell::new(0u32))
                // .data(AppData::new(0u32))
                .wrap(middleware::Logger::default())
                .service(resource("/").route(get().to(router::index)))
                .service(resource("/christmas").route(get().to(router::christmas)))
                .service(
                    resource("/triplets")
                        .route(get().to(router::triplets))
                        .route(post().to(router::generate_triplets)),
                )
                .service(
                    resource("/multipart_image")
                        .route(get().to(router::multipart_image))
                        .route(post().to_async(router::load_image)),
                )
                .service(
                    resource("/fibonacci")
                        .route(get().to(router::fibonacci))
                        .route(post().to(router::fibonacci_culc)),
                )
                .service(resource("/db").route(get().to(router::db)))
                .service(
                    resource("/db/user")
                        .route(get().to(router::get_user))
                        .route(post().to(router::save_user)),
                )
                .service(resource("/convert").route(get().to(router::convert)))
                .service(resource("/c2f").route(post().to(router::c2_f)))
                .service(resource("/f2c").route(post().to(router::f2_c)))
                // redirect
                .service(resource("/test").route(
                    get().to(|| HttpResponse::Found().header(header::LOCATION, "/").finish()),
                ))
                .service(fs::Files::new("/", "/static/**/*"))
                // set default route to 404
                .default_service(route().to(router::p404))
        })
        .bind(path)
        .expect(&format!("{}{}", "Could not bind to port ", &self.port));

        println!("Server is running on {}:{}", &self.address, &self.port);


        match server.run() {
            //match is executed when server is shut down
            Ok(_) => println!("\nServer is gracefully shut down"),
            Err(why) => println!("There was a problem stoping the server: {}", why),
        };
    }
}
