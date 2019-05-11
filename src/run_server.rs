use actix_web::http::{header, Method};
use actix_web::middleware::session;
use actix_web::{fs, middleware, pred, server::HttpServer, App, HttpResponse};
use listenfd::ListenFd;
use crate::router as router;

pub struct Server
{
    pub name: String,
    pub port: String,
}

impl Server
{
    pub fn start(&mut self) {
        //init autoreload additional sockets
        let mut listenfd = ListenFd::from_env();

        let mut server = HttpServer::new(||
            App::new()
                .middleware(middleware::Logger::default())
                .middleware(session::SessionStorage::new(
                    session::CookieSessionBackend::signed(&[0; 32]).secure(false),
                ))
                .resource("/", |r| r.get().f(router::index))
                .resource("/favicon", |r| r.f(router::favicon))
                .resource("/triplets", |r| r.f(router::triplets))
                .resource("/generate_triplets", |r| r.f(router::generate_triplets))
                .resource("/multipart_image", |r| r.get().f(router::multipart_image))
                .resource("/load_image", |r| r.post().f(router::load_image))
                .resource("/calculate", |r| r.get().f(router::calculate))
                // redirect
                .resource("/test", |r| {
                    r.get().f(|req| {
                        println!("{:?}", req);
                        HttpResponse::Found()
                            .header(header::LOCATION, "/")
                            .finish()
                    })
                })
                //static files
                .handler("/static", fs::StaticFiles::new("static").unwrap())
                // default
                .default_resource(|r| {
                    // 404 for GET request
                    r.method(Method::GET).f(router::p404);

                    // all requests that are not `GET`
                    r.route()
                        .filter(pred::Not(pred::Get()))
                        .f(|_| HttpResponse::MethodNotAllowed());
                })
        );

        let path = "127.0.0.1:".to_owned() + &self.port;
        server = if let Some(l) =
        listenfd.take_tcp_listener(0).unwrap() {
            server.listen(l)
        } else {
            server
                .bind(path)
                .expect(&format!("{}{}", "Could not bind to port ", &self.port))
        };

        server.run();
        println!("Server is running on 127.0.0.1:{}", &self.port);
    }
}
