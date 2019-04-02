// use futures;
// use env_logger;
// use actix_web::http::{header, Method};
// use actix_web::middleware::session;
// use actix_web::{fs, middleware, pred, server::HttpServer, App, HttpResponse};
// use listenfd::ListenFd;
// use router;

// pub struct Server {
//     name: String,
//     instance: HttpServer
// }

// impl Server {
//     fn new(&mut self, name: &str)-> Server {
//         self.name = name.to_owned();

//             //init autoreload additional sockets
//     let mut listenfd = ListenFd::from_env();

//             self.instance = HttpServer::new(|| 
//         App::new()
//             .middleware(middleware::Logger::default())
//             .middleware(session::SessionStorage::new(
//                 session::CookieSessionBackend::signed(&[0; 32]).secure(false),
//             ))
//             .resource("/favicon", |r| r.f(router::favicon))
//             .resource("/", |r| r.get().f(router::index))
//             .resource("/detail", |r| r.get().f(router::detail))
//             .resource("/calculate", |r| r.get().f(router::calculate))
//             // redirect
//             .resource("/test", |r| {
//                 r.get().f(|req| {
//                     println!("{:?}", req);
//                     HttpResponse::Found()
//                         .header(header::LOCATION, "/")
//                         .finish()
//                 })
//             })
//             //static files
//             .handler("/static", fs::StaticFiles::new("static").unwrap())
//             // default
//             .default_resource(|r| {
//                 // 404 for GET request
//                 r.method(Method::GET).f(router::p404);

//                 // all requests that are not `GET`
//                 r.route()
//                     .filter(pred::Not(pred::Get()))
//                     .f(|_| HttpResponse::MethodNotAllowed());
//             })
            
//     ).bind("127.0.0.1:3000")
//             .expect("Could not bind to port 3000")
//             .start();



//     self.instance.run();
    
//         *self
//     }
// }