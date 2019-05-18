use crate::controllers::{pythagorian_triplets, get_christmas_lyrics, Fibonacci,
                         Triplet, fahrenheit_to_celsius, celsius_to_fahrenheit};
use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use web::{Form, Data, Query};
use std::collections::HashMap;
use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*"));
        tera.autoescape_on(vec!["html", ".sql"]);
        tera
    };
}

///main page handler
pub fn index(query: Query<HashMap<String, String>>) -> Result<HttpResponse, Error> {
    let mut counter = 1;
    // if let Some(count) = req.app_data().get::<i32>("counter")? {
    //     println!("SESSION value: {}", count);
    //     counter = count + 1;
    // }

    // set counter to session
    // req.session().set("counter", counter)?;

    let mut ctx = Context::new();
    ctx.insert("counter", &counter);
    render_with_ctx("pages/index.html", ctx)
}


///404 page
pub fn p404(t: Data<Tera>) -> Result<HttpResponse, Error> {
    let s = t
        .render("pages/404.html", &Context::new())
        .map_err(|_| error::ErrorInternalServerError("Check template paths"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

///triplets page
pub fn triplets() -> Result<HttpResponse, Error> {
    render_page("pages/triplets.html")
}

///load image page
pub fn multipart_image() -> Result<HttpResponse, Error> {
    render_page("pages/multipart_image.html")
}

///fibonacci page
pub fn fibonacci() -> Result<HttpResponse, Error> {
    render_page("pages/multipart_image.html")
}

///convert page
pub fn convert() -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    ctx.insert("type", "C");

    render_page("pages/temp_convert.html")
}


///convert result
/// for both cases
pub fn convert_result(ctx: Context)
                      -> Result<HttpResponse, Error> {
    render_with_ctx("pages/temp_convert.html", ctx)
}


/// christmas lyrics
/// get christmas lyrics and send it to the screen
pub fn christmas() -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let lyrics = get_christmas_lyrics();
//    println!("lyrics {}", lyrics);
    ctx.insert("lyrics", &lyrics);

    render_with_ctx("pages/christmas.html", ctx)
}

///triplets
/// TODO: get actual form data
pub fn generate_triplets(query: Query<HashMap<String, String>>)
                         -> Result<HttpResponse, Error> {
    println!("params {:?}", query.get("temp"));
    let n = "2";
    let triplet: Triplet = pythagorian_triplets(n);
    let mut ctx = Context::new();
    ctx.insert("time", &triplet.time().to_string());
    ctx.insert("triplet", &triplet.body());

    println!("triplet {:?} time {:?} ", &triplet.body(), &triplet.time());

    render_with_ctx("pages/triplets.html", ctx)
}

///process multipart image file
/// TODO: get actual form data
pub fn load_image(query: Query<HashMap<String, String>>) -> Result<HttpResponse, Error> {
    // println!("state {:?} body {:#?}", req.state(), req.request().headers());
    println!("load image process initiated");
    let mut ctx = Context::new();

    render_with_ctx("pages/multipart_image.html", ctx)
}

/// fibonacci
pub fn fibonacci_culc(params: Form<Fibonacci>, query: Query<HashMap<String, String>>) -> Result<HttpResponse, Error> {
    // println!("params {}", params);
    let mut ctx = Context::new();
    // ctx.insert("number", params.number);

    render_with_ctx("pages/triplets.html", ctx)
}

///celsius to fahrenheit
pub fn c2_f(query: Query<HashMap<String, String>>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let temp = celsius_to_fahrenheit("0");
    // println!("state {:?} body {:#?}", req.state(), req.request().headers());

    ctx.insert("temp", &temp);
    ctx.insert("type", "C");

    convert_result(ctx)
}

//fahrenheit to celsius
pub fn f2_c(query: Query<HashMap<String, String>>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let temp = fahrenheit_to_celsius("0");
    println!("temp {}", temp);
    ctx.insert("temp", &temp);
    ctx.insert("type", "F");

    convert_result(ctx)
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
