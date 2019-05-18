use crate::router;
use actix_web::http::header;
use actix_files as fs;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use std::io;
// use listenfd::ListenFd;

use tera;
use web::{get, post, resource, route};

pub struct Server {
    pub name: String,
    pub port: String,
}

impl Server {
    pub fn start(&mut self, path: &str) -> Result<(), io::Error> {
        //init autoreload additional sockets
        // let listenfd = ListenFd::from_env();
        let path = path.to_owned() + ":" + &self.port;

        let server = HttpServer::new(|| {
            let mut tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*"));
            tera.autoescape_on(vec!["html", ".sql"]);

            App::new()
                .data(tera)
                .wrap(middleware::Logger::default())
                .service(resource("/").route(get().to(router::index)))
                .service(resource("/triplets").route(get().to(router::triplets)))
                .service(resource("/christmas").route(get().to(router::christmas)))
                .service(resource("/generate_triplets").route(get().to(router::generate_triplets)))
                .service(resource("/multipart_image").route(get().to(router::multipart_image)))
                .service(resource("/load_image").route(get().to(router::load_image)))
                .service(resource("/load_image").route(post().to(router::load_image)))
                .service(resource("/fibonacci").route(get().to(router::fibonacci)))
                .service(resource("/fibonacci").route(post().to(router::fibonacci)))
                .service(resource("/convert").route(get().to(router::convert)))
                .service(resource("/c2f").route(post().to(router::c2_f)))
                .service(resource("/f2c").route(post().to(router::f2_c)))
                // redirect
                .service(resource("/test").route(
                    get().to(|| HttpResponse::Found().header(header::LOCATION, "/").finish()),
                ))
                .service(fs::Files::new("/", "./static/**/*"))
                // default
                .default_service(route().to(router::p404))
        })
            .bind(path)
            .expect(&format!("{}{}", "Could not bind to port ", &self.port));

        println!("Server is running on 127.0.0.1:{}", &self.port);
        server.run()
    }
}
