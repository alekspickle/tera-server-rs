// use crate::controllers::AppData;
use crate::router;

use std::cell::Cell;

use actix_files as fs;
use actix_web::http::header;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};

use web::{get, post, resource, route};


pub struct Server {
    pub name: String,
    pub address: String,
    pub port: String,

}

impl Server {
    pub fn start(&mut self) {
        let path = self.address.to_owned() + ":" + &self.port;

        let server = HttpServer::new(|| {
            let mut tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*"));
            tera.autoescape_on(vec!["html", ".sql"]);

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

        println!("Server is running on {}:{}", &self.address, &self.port);

        match server.run() {
            Ok(_) => println!("Server is gracefully shut down"),
            Err(why) => println!("There was a problem stoping the server: {}", why),
        };
    }
}
