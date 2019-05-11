use crate::controllers::{pythagorian_triplets, Triplet};
use actix_web::middleware::session::RequestSession;
use actix_web::{error, fs, Error, HttpMessage, HttpRequest, HttpResponse};
use futures::future::Future;
use tera::{Context, Tera};

struct PythagorianForm {
    n: i32
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*"));
        tera.autoescape_on(vec!["html", ".sql"]);
        tera
    };
}

fn show_request(req: &actix_web::HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    Box::new(req.body().map_err(|e| e.into()).map(move |f| {
        actix_web::HttpResponse::Ok()
            .content_type("text/plain")
            .body(f)
    }))
}

/// favicon handler
pub fn favicon(req: &HttpRequest) -> Result<fs::NamedFile, Error> {
    println!("Where is favicon?");
    Ok(fs::NamedFile::open("assets/favicon.svg")?)
}

///main page handler
pub fn index(req: &HttpRequest) -> Result<HttpResponse, Error> {
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

///triplets page
pub fn triplets(req: &HttpRequest) -> Result<HttpResponse, Error> {
    let mut _ctx = Context::new();

    render_page("pages/triplets.html")
}

///load image functionality
pub fn load_image(req: &HttpRequest) -> Result<HttpResponse, Error> {
    let mut _ctx = Context::new();

    render_page("pages/load_images.html")
}

///triplets
pub fn generate_triplets(req: &HttpRequest) -> Result<HttpResponse, Error> {
    let n = "2";
    show_request(req);
    println!("state {:?} body {:?}", req.state(), req.body().wait());
    let triplet: Triplet = pythagorian_triplets(n);
    let mut ctx = Context::new();
    ctx.insert("triplet", &triplet.body());
    ctx.insert("time", &triplet.time());

    println!("triplet {:?} time {:?} ", &triplet.body(), &triplet.time());

    render_page("pages/triplets.html")
}

//calculations
pub fn calculate(req: &HttpRequest) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let calculated: u32 = 10 + 9876;
    ctx.insert("calculated", &calculated);
    render_with_ctx("pages/calculate.html", ctx)
}

///404
pub fn p404(req: &HttpRequest) -> Result<HttpResponse, Error> {
    let mut _ctx = Context::new();
    render_page("pages/404.html")
}

///function, that renders template with params
pub fn render_with_ctx(template: &str, ctx: Context) -> Result<HttpResponse, Error> {
    let s = TEMPLATES
        .render(template, &ctx.to_owned())
        .map_err(|_| error::ErrorInternalServerError("Check template paths"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

///function, that renders template without params
pub fn render_page(template: &str) -> Result<HttpResponse, Error> {
    let s = TEMPLATES
        .render(template, &Context::new())
        .map_err(|_| error::ErrorInternalServerError("Check template paths"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
