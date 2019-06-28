//! ## Process routes.
//! Make use of controller logic. Derive all output to the screen.
//!
//!
//!
//!
//!

use crate::controllers::{
    celsius_to_fahrenheit, fahrenheit_to_celsius, fibonacci_number, get_christmas_lyrics,
    pythagorian_triplets, visit_dirs, ConvertForm, NForm, Triplet,
};
use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use future::{err, Either};
use futures::{future, Future, Stream};
use tera::{Context, Tera};
use web::{Data, Form};

use std::{
    cell::Cell,
    collections::HashMap,
    env,
    fs::{self, DirEntry, File},
    io::{ErrorKind, Write},
    path::Path,
    process::Command,
    thread,
    time::{Duration, SystemTime},
};

///main page handler
///TODO: use custom struct instead of Cell<u32> in Data
pub fn index(
    t: Data<Tera>,
    data: Data<Cell<u32>>,
    // data: Data<AppData>,
) -> Result<HttpResponse, Error> {
    // set counter to session
    data.set(data.get() + 1);

    if env::vars()
        .collect::<HashMap<String, String>>()
        .get("LOGGER")
        .is_some()
    {
        println!("cargo dir {}", env!("CARGO_MANIFEST_DIR"));
    }

    render_page("pages/index.html", t)
}

///404 page
pub fn p404(t: Data<Tera>) -> Result<HttpResponse, Error> {
    render_page("pages/404.html", t)
}

///triplets page
pub fn triplets(t: Data<Tera>) -> Result<HttpResponse, Error> {
    render_page("pages/triplets.html", t)
}

///multipart request image page
pub fn multipart_image(t: Data<Tera>) -> Result<HttpResponse, Error> {
    render_page("pages/multipart_image.html", t)
}

///multipart request success image page
pub fn multipart_success(t: Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    ctx.insert("success", "Successfully uploaded an image!");

    render_with_ctx("pages/multipart_image.html", ctx, t)
}

///fibonacci page
pub fn fibonacci(t: Data<Tera>) -> Result<HttpResponse, Error> {
    render_page("pages/fibonacci.html", t)
}

///convert page
pub fn convert(t: Data<Tera>) -> Result<HttpResponse, Error> {
    render_page("pages/temp_convert.html", t)
}

/// convert result page
/// for both cases
pub fn convert_result(t: Data<Tera>, ctx: Context) -> Result<HttpResponse, Error> {
    render_with_ctx("pages/temp_convert.html", ctx, t)
}

/// Christmas lyrics route
/// Get christmas lyrics and send it to the screen
pub fn christmas(t: Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let lyrics = get_christmas_lyrics();
    ctx.insert("lyrics", &lyrics);

    render_with_ctx("pages/christmas.html", ctx, t)
}

///Triplets route
pub fn generate_triplets(t: Data<Tera>, data: Form<NForm>) -> Result<HttpResponse, Error> {
    let triplet: Triplet = pythagorian_triplets(&data.n);
    let mut ctx = Context::new();
    ctx.insert("time", &triplet.time().to_string());
    ctx.insert("triplet", &triplet.body());

    render_with_ctx("pages/triplets.html", ctx, t)
}

///process multipart image file
pub fn load_image(
    t: Data<Tera>,
    multipart: Multipart,
) -> impl Future<Item = Result<HttpResponse, Error>, Error = Error> {
    println!("load image process initiated. ");

    //actually upload it to the server
    upload(t, multipart)
}

/// fibonacci
pub fn fibonacci_culc(t: Data<Tera>, data: Form<NForm>) -> Result<HttpResponse, Error> {
    let n = data.n.clone();
    let mut ctx = Context::new();
    let r = fibonacci_number(n);
    let result = format!("{}{}", r.0, r.1);
    ctx.insert("number", &result);

    render_with_ctx("pages/fibonacci.html", ctx, t)
}

///celsius to fahrenheit
pub fn c2_f(t: Data<Tera>, data: Form<ConvertForm>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let temp = celsius_to_fahrenheit(&data.temp);
    ctx.insert("temp", &temp);

    convert_result(t, ctx)
}

//fahrenheit to celsius
pub fn f2_c(t: Data<Tera>, data: Form<ConvertForm>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let temp = fahrenheit_to_celsius(&data.temp);
    ctx.insert("temp", &temp);

    convert_result(t, ctx)
}

pub fn save_file(field: Field) -> impl Future<Item = i64, Error = Error> {
    let base = "downloads/upload_".to_owned();
    println!("file {:?}", field);
    let code = match SystemTime::now().duration_since(<std::time::SystemTime>::UNIX_EPOCH) {
        Ok(now) => now,
        Err(_) => Duration::new(0, 0),
    };
    let file_path_string = base.clone() + &code.as_millis().to_string() + ".png";
    // let post_str = &file_path_string.clone();

    let file = match File::create(file_path_string.clone()) {
        Ok(file) => file,
        Err(match_e) => {
            if match_e.kind() == ErrorKind::NotFound {
                println!("Create 'downloads' directory in the root of the project please");
                File::create(&file_path_string).expect("Second file create attempt failed")
            } else {
                return Either::A(err(error::ErrorInternalServerError(match_e)));
            }
        }
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
            .map(|(_, acc)| {
                // post(post_str);
                acc
            })
            .map_err(|e| {
                println!("save_file failed, {:?}", e);
                error::ErrorInternalServerError(e)
            }),
    )
}

pub fn upload(
    t: Data<Tera>,
    multipart: Multipart,
) -> impl Future<Item = Result<HttpResponse, Error>, Error = Error> {
    let copy = &multipart;

    multipart
        .map_err(error::ErrorInternalServerError)
        .map(|field| save_file(field).into_stream())
        .flatten()
        .collect()
        .map(|_sizes| multipart_success(t))
        .map_err(|e| {
            println!("failed: {}", e);
            e
        })
}

///function, that renders template with params
pub fn render_with_ctx(template: &str, ctx: Context, t: Data<Tera>) -> Result<HttpResponse, Error> {
    let s = t
        .render(template, &ctx.to_owned())
        .map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

///function, that renders template without params
pub fn render_page(template: &str, t: Data<Tera>) -> Result<HttpResponse, Error> {
    let s = t
        .render(template, &Context::new())
        .map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
