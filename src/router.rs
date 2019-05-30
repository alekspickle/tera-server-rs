//! ## Process routes.
//! Make use of controller logic. Derive all output to the screen.
//!
//!
//!
//!
//!


use crate::controllers::{
    celsius_to_fahrenheit, fahrenheit_to_celsius, fibonacci_number, get_christmas_lyrics,
    pythagorian_triplets, ConvertForm, NForm, Triplet,
};
use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{error, web, Error, HttpResponse};
use futures::future::{err, Either};
use futures::{Future, Stream};


use std::cell::Cell;
use std::fs::File;
use std::io::{ErrorKind, Write};
use std::time::{Duration, SystemTime};
use tera::{Context, Tera};
use web::{Data, Form};


lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*"));
        tera.autoescape_on(vec!["html", ".sql"]);
        tera
    };
}

///main page handler
///TODO: use custom struct instead of Cell<u32> in Data
pub fn index(
    data: Data<Cell<u32>>,
    // data: Data<AppData>,
) -> Result<HttpResponse, Error> {
    // set counter to session
    data.set(data.get() + 1);

    render_page("pages/index.html")
}

///404 page
pub fn p404(_t: Data<Tera>) -> Result<HttpResponse, Error> {
    render_page("pages/404.html")
}

///triplets page
pub fn triplets(_t: Data<Tera>) -> Result<HttpResponse, Error> {
    render_page("pages/triplets.html")
}

///multipart request image page
pub fn multipart_image() -> Result<HttpResponse, Error> {
    render_page("pages/multipart_image.html")
}

///multipart request success image page
pub fn multipart_success() -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    ctx.insert("success", "Successfully uploaded an image!");

    render_with_ctx("pages/multipart_image.html", ctx)
}

///fibonacci page
pub fn fibonacci(_t: Data<Tera>) -> Result<HttpResponse, Error> {
    render_page("pages/fibonacci.html")
}

///convert page
pub fn convert(_t: Data<Tera>) -> Result<HttpResponse, Error> {
    render_page("pages/temp_convert.html")
}


/// convert result page
/// for both cases
pub fn convert_result(_t: Data<Tera>, ctx: Context) -> Result<HttpResponse, Error> {
    render_with_ctx("pages/temp_convert.html", ctx)
}


/// Christmas lyrics route
/// Get christmas lyrics and send it to the screen
pub fn christmas(_t: Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let lyrics = get_christmas_lyrics();
    ctx.insert("lyrics", &lyrics);

    render_with_ctx("pages/christmas.html", ctx)
}

///Triplets route
pub fn generate_triplets(data: Form<NForm>) -> Result<HttpResponse, Error> {
    let triplet: Triplet = pythagorian_triplets(&data.n);
    let mut ctx = Context::new();
    ctx.insert("time", &triplet.time().to_string());
    ctx.insert("triplet", &triplet.body());

    render_with_ctx("pages/triplets.html", ctx)
}

///process multipart image file
pub fn load_image(
    multipart: Multipart,
) -> impl Future<Item = Result<HttpResponse, Error>, Error = Error> {
    println!("load image process initiated.  ",);

    //actually upload it to the server
    upload(multipart)


}

/// fibonacci
pub fn fibonacci_culc(_t: Data<Tera>, data: Form<NForm>) -> Result<HttpResponse, Error> {
    let n = data.n.clone();
    let mut ctx = Context::new();
    let number = fibonacci_number(n);
    ctx.insert("number", &number);

    render_with_ctx("pages/fibonacci.html", ctx)
}

///celsius to fahrenheit
pub fn c2_f(_t: Data<Tera>, data: Form<ConvertForm>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let temp = celsius_to_fahrenheit(&data.temp);
    ctx.insert("temp", &temp);

    convert_result(_t, ctx)
}

//fahrenheit to celsius
pub fn f2_c(_t: Data<Tera>, data: Form<ConvertForm>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let temp = fahrenheit_to_celsius(&data.temp);
    ctx.insert("temp", &temp);

    convert_result(_t, ctx)
}


pub fn save_file(field: Field) -> impl Future<Item = i64, Error = Error> {
    let base = "downloads/upload_".to_owned();
    let code = match SystemTime::now().duration_since(<std::time::SystemTime>::UNIX_EPOCH) {
        Ok(now) => now,
        Err(_) => Duration::new(0, 0),
    };
    let file_path_string = base.clone() + &code.as_millis().to_string() + ".png";
    let file = match File::create(file_path_string.clone()) {
        Ok(file) => file,
        Err(match_e) => {
            if match_e.kind() == ErrorKind::NotFound {
                println!("Create 'downloads' directory in the root of the project please");
                File::create(file_path_string).expect("Second file create attempt failed")
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
            .map(|(_, acc)| acc)
            .map_err(|e| {
                println!("save_file failed, {:?}", e);
                error::ErrorInternalServerError(e)
            }),
    )
}

pub fn upload(
    multipart: Multipart,
) -> impl Future<Item = Result<HttpResponse, Error>, Error = Error> {
    multipart
        .map_err(error::ErrorInternalServerError)
        .map(|field| save_file(field).into_stream())
        .flatten()
        .collect()
        .map(|_sizes| multipart_image())
        .map_err(|e| {
            println!("failed: {}", e);
            e
        })
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