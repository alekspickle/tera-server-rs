use crate::controllers::{
    celsius_to_fahrenheit, fahrenheit_to_celsius, fibonacci_number, get_christmas_lyrics,
    pythagorian_triplets, ConvertForm, NForm, Triplet,
};
use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{error, web, Error, HttpResponse};
use futures::future::{err, Either};
use futures::{Future, Stream};
use std::cell::Cell;

use std::fs;
use std::io::Write;
use std::collections::HashMap;
use tera::{Context, Tera};
use web::{Data, Form, Query};

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
    render_with_ctx("pages/index.html", ctx)
}

///404 page
pub fn p404(t: Data<Tera>) -> Result<HttpResponse, Error> {
    render_page("pages/404.html")
}

///triplets page
pub fn triplets(t: Data<Tera>) -> Result<HttpResponse, Error> {
    render_page("pages/triplets.html")
}

///load image page
pub fn multipart_image(t: Data<Tera>) -> Result<HttpResponse, Error> {
    render_page("pages/multipart_image.html")
}

///fibonacci page
pub fn fibonacci(t: Data<Tera>) -> Result<HttpResponse, Error> {
    render_page("pages/fibonacci.html")
}

///convert page
pub fn convert(t: Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    ctx.insert("type", "C");

    render_page("pages/temp_convert.html")
}


/// convert result
/// for both cases
pub fn convert_result(t: Data<Tera>, ctx: Context) -> Result<HttpResponse, Error> {
    render_with_ctx("pages/temp_convert.html", ctx)
}


/// christmas lyrics
/// get christmas lyrics and send it to the screen
pub fn christmas(t: Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let lyrics = get_christmas_lyrics();
    ctx.insert("lyrics", &lyrics);

    render_with_ctx("pages/christmas.html", ctx)
}

///triplets
/// TODO: get actual form data
pub fn generate_triplets(data: Form<NForm>) -> Result<HttpResponse, Error> {
    println!("generate query {:?}", data.n);
    let triplet: Triplet = pythagorian_triplets(&data.n);
    let mut ctx = Context::new();
    ctx.insert("time", &triplet.time().to_string());
    ctx.insert("triplet", &triplet.body());

    println!("triplet {:?} time {:?} ", &triplet.body(), &triplet.time());

    render_with_ctx("pages/triplets.html", ctx)
}

///process multipart image file
/// TODO: get actual form data
pub fn load_image(t: Data<Tera>) -> Result<HttpResponse, Error> {
    println!("load image process initiated");
    let ctx = Context::new();

    render_with_ctx("pages/multipart_image.html", ctx)
}

/// fibonacci
pub fn fibonacci_culc(t: Data<Tera>, data: Form<NForm>) -> Result<HttpResponse, Error> {
    let n = data.n.clone();
    let mut ctx = Context::new();
    let number = fibonacci_number(n);
    ctx.insert("number", &number);

    render_with_ctx("pages/fibonacci.html", ctx)
}

///celsius to fahrenheit
pub fn c2_f(t: Data<Tera>, data: Form<ConvertForm>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let temp = celsius_to_fahrenheit(&data.temp);
    println!("data {:?}", data);

    ctx.insert("temp", &temp);
    ctx.insert("type", "C");

    convert_result(t, ctx)
}

//fahrenheit to celsius
pub fn f2_c(t: Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let temp = fahrenheit_to_celsius("0");
    println!("temp {}", temp);
    ctx.insert("temp", &temp);
    ctx.insert("type", "F");

    convert_result(t, ctx)
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

pub fn save_file(field: Field) -> impl Future<Item = i64, Error = Error> {
    let file_path_string = "upload.png";
    let file = match fs::File::create(file_path_string) {
        Ok(file) => file,
        Err(e) => return Either::A(err(error::ErrorInternalServerError(e))),
    };
    Either::B(
        field
            .fold((file, 0i64), move |(mut file, mut acc), bytes| {
                // fs operations are blocking, we have to execute writes
                // on threadpool
                web::block(move || {
                    file.write_all(bytes.as_ref()).map_err(|e| {
                        println!("file.write_all failed: {:?}", e);
                        MultipartError::Payload(error::PayloadError::Io(e))
                    })?;
                    acc += bytes.len() as i64;
                    Ok((file, acc))
                })
                .map_err(|e: error::BlockingError<MultipartError>| match e {
                    error::BlockingError::Error(e) => e,
                    error::BlockingError::Canceled => MultipartError::Incomplete,
                })
            })
            .map(|(_, acc)| acc)
            .map_err(|e| {
                println!("save_file failed, {:?}", e);
                error::ErrorInternalServerError(e)
            }),
    )
}

pub fn upload(
    multipart: Multipart,
    counter: web::Data<Cell<usize>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    counter.set(counter.get() + 1);
    println!("{:?}", counter.get());

    multipart
        .map_err(error::ErrorInternalServerError)
        .map(|field| save_file(field).into_stream())
        .flatten()
        .collect()
        .map(|sizes| HttpResponse::Ok().json(sizes))
        .map_err(|e| {
            println!("failed: {}", e);
            e
        })
}