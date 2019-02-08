#[macro_use]
extern crate tera;
#[macro_use]
extern crate lazy_static;

use actix_web::http::{header, Method, StatusCode};
use actix_web::middleware::session::{self, RequestSession};
use actix_web::{error, fs, http, middleware, pred, server, App, Error, HttpRequest, HttpResponse};
use env_logger;
use listenfd::ListenFd;
use tera::{Context, Tera};
mod router;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*"));
        tera.autoescape_on(vec!["html", ".sql"]);
        tera
    };
}

/// favicon handler
fn favicon(req: &HttpRequest) -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open("assets/favicon.ico")?)
}

///main page handler
fn index(req: &HttpRequest) -> Result<HttpResponse, Error> {
    let mut counter = 1;
    if let Some(count) = req.session().get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
    }

    // set counter to session
    req.session().set("counter", counter)?;

    let mut ctx = Context::new();
    ctx.insert("counter", &counter);
    render_with_ctx("pages/index.html", ctx)
}

fn detail(req: &HttpRequest) -> Result<HttpResponse, Error> {
    let mut _ctx = Context::new();

    render_page("pages/detail.html")
}
fn calculate(req: &HttpRequest) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let calculated: u32 = 10 + 9876;
    ctx.insert("calculated", &calculated);
    render_with_ctx("pages/calculate.html", ctx)
}

fn p404(req: &HttpRequest) -> Result<HttpResponse, Error> {
    let mut _ctx = Context::new();
    render_page("pages/404.html")
}

fn render_with_ctx(template: &str, ctx: Context) -> Result<HttpResponse, Error> {
    let s = TEMPLATES
        .render(template, &ctx.to_owned())
        .map_err(|_| error::ErrorInternalServerError("Check template paths"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn render_page(template: &str) -> Result<HttpResponse, Error> {
    let s = TEMPLATES
        .render(template, &Context::new())
        .map_err(|_| error::ErrorInternalServerError("Check template paths"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn main() {
    //init logger
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    //init autoreload additional sockets
    let mut listenfd = ListenFd::from_env();

    let mut server = server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .middleware(session::SessionStorage::new(
                session::CookieSessionBackend::signed(&[0; 32]).secure(false),
            ))
            .handler("/static", fs::StaticFiles::new("static").unwrap())
            .resource("/favicon", |r| r.get().f(favicon))
            .resource("/", |r| r.get().f(index))
            .resource("/detail", |r| r.get().f(detail))
            .resource("/calculate", |r| r.get().f(calculate))
            // redirect
            .resource("/test", |r| {
                r.get().f(|req| {
                    println!("{:?}", req);
                    HttpResponse::Found()
                        .header(header::LOCATION, "pages/index.html")
                        .finish()
                })
            })
            // default
            .default_resource(|r| {
                // 404 for GET request
                r.method(Method::GET).f(p404);

                // all requests that are not `GET`
                r.route()
                    .filter(pred::Not(pred::Get()))
                    .f(|_| HttpResponse::MethodNotAllowed());
            })
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server
            .bind("127.0.0.1:3000")
            .expect("Could not bind to port 3000")
    };

    server.run()
}
