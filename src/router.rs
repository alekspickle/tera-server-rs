use crate::controllers::{pythagorian_triplets, get_christmas_lyrics, Triplet, fahrenheit_to_celsius, celsius_to_fahrenheit};
use actix_web::middleware::session::RequestSession;
use actix_web::{error, fs, Error, HttpMessage, HttpRequest, HttpResponse};
use futures::future::Future;
use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*"));
        tera.autoescape_on(vec!["html", ".sql"]);
        tera
    };
}

fn _show_request(req: &actix_web::HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    Box::new(req.body().map_err(|e| e.into()).map(move |f| {
        actix_web::HttpResponse::Ok()
            .content_type("text/plain")
            .body(f)
    }))
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


///404 page
pub fn p404(req: &HttpRequest) -> Result<HttpResponse, Error> {
    render_page("pages/404.html")
}

///triplets page
pub fn triplets(req: &HttpRequest) -> Result<HttpResponse, Error> {
    render_page("pages/triplets.html")
}

///load image page
pub fn multipart_image(req: &HttpRequest) -> Result<HttpResponse, Error> {
    render_page("pages/multipart_image.html")
}
///convert page
pub fn convert(req: &HttpRequest) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    ctx.insert("type", "C");

    render_page("pages/temp_convert.html")
}


///triplets result
pub fn triplets_result(req: &HttpRequest, ctx: Context) -> Result<HttpResponse, Error> {
    render_with_ctx("pages/triplets.html", ctx)
}

///convert result
pub fn convert_result(req: &HttpRequest, ctx: Context) -> Result<HttpResponse, Error> {
    render_with_ctx("pages/temp_convert.html", ctx)
}


/// christmas lyrics
/// get christmas lyrics and send it to the screen
pub fn christmas(req: &HttpRequest) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let lyrics = get_christmas_lyrics();
//    println!("lyrics {}", lyrics);
    ctx.insert("lyrics", &lyrics);

    render_with_ctx("pages/christmas.html", ctx)
}

///triplets
/// TODO: get actual form data
pub fn generate_triplets(req: &HttpRequest) -> Result<HttpResponse, Error> {
    let n = "2";
    let triplet: Triplet = pythagorian_triplets(n);
    let mut ctx = Context::new();
    ctx.insert("time", &triplet.time().to_string());
    ctx.insert("triplet", &triplet.body());

    println!("triplet {:?} time {:?} ", &triplet.body(), &triplet.time());

    triplets_result(req, ctx)
}

///process multipart image file
/// TODO: get actual form data
pub fn load_image(req: &HttpRequest) -> Result<HttpResponse, Error> {
    println!("state {:?} body {:#?}", req.state(), req.request().headers());
    println!("load image process initiated");
    let mut _ctx = Context::new();

    multipart_image(req)
}

///celsius to fahrenheit
pub fn c2_f(req: &HttpRequest) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let temp = celsius_to_fahrenheit("0");
    println!("state {:?} body {:#?}", req.state(), req.request().headers());

    ctx.insert("temp", &temp);
    ctx.insert("type", "C");

    convert_result(req, ctx)
}

//fahrenheit to celsius
pub fn f2_c(req: &HttpRequest) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let temp = fahrenheit_to_celsius("0");
    println!("temp {}", temp);
    ctx.insert("temp", &temp);
    ctx.insert("type", "F");

    convert_result(req, ctx)
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
