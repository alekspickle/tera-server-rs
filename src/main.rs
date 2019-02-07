#[macro_use]
extern crate tera;

use actix_web;
use actix_web::{error, fs, http, middleware, server, App, Error, HttpResponse, State};
use env_logger;
use listenfd::ListenFd;
mod router;

struct AppState {
    template: tera::Tera,
}

fn index(state: State<AppState>) -> Result<HttpResponse, Error> {
    let mut _ctx = tera::Context::new();
    render_page(state, "pages/index.html")
}

fn detail(state: State<AppState>) -> Result<HttpResponse, Error> {
    let mut _ctx = tera::Context::new();

    render_page(state, "pages/detail.html")
}
fn calculate(state: State<AppState>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let calculated: u32 = 10 + 9876;
    ctx.insert("calculated", &calculated.to_owned());
    render_with_ctx(state, "pages/calculate.html", ctx)
}

fn p404(state: State<AppState>) -> Result<HttpResponse, Error> {
    let mut _ctx = tera::Context::new();
    render_page(state, "pages/404.html")
}

fn render_with_ctx(
    state: State<AppState>,
    template: &str,
    ctx: tera::Context,
) -> Result<HttpResponse, Error> {
    let s = state
        .template
        .render(template, &ctx.to_owned())
        .map_err(|_| error::ErrorInternalServerError("Check template paths"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn render_page(state: State<AppState>, template: &str) -> Result<HttpResponse, Error> {
    let s = state
        .template
        .render(template, &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Check template paths"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn main() {
    //init logger
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    //init autoreload
    let mut listenfd = ListenFd::from_env();

    let mut server = server::new(|| {
        let tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*"));

        App::with_state(AppState { template: tera })
            .middleware(middleware::Logger::default())
            .handler("/static", fs::StaticFiles::new("static").unwrap())
            .resource("/", |r| r.method(http::Method::GET).with(index))
            .resource("/detail", |r| r.method(http::Method::GET).with(detail))
            .resource("/calculate", |r| {
                r.method(http::Method::GET).with(calculate)
            })
            .default_resource(|r| r.method(http::Method::GET).with(p404))
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
