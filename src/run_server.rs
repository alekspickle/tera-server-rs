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
                .resource("/favicon", |r| r.f(router::favicon))
                .resource("/", |r| r.get().f(router::index))
//                .resource("/detail", |r| r.get().f(router::detail))
                .resource("/image", |r| r.get().f(router::load_image))
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
//    pub fn run() -> () {
//        let path = "127.0.0.1:".to_owned() + port;
//        self.instance = if let Some(l) =
//        self.listenfd.take_tcp_listener(0).unwrap() {
//            self.instance.listen(l)
//        } else {
//            self.instance
//                .bind(path)
//                .expect(&format!("{}{}","Could not bind to port ",port))
//        };
//
//        self.instance.run();
//        println!("Server is running on 127.0.0.1:{}",port);
//    }
}