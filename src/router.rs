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
use std::time::SystemTime;
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

///load image page
pub fn multipart_image() -> Result<HttpResponse, Error> {
    render_page("pages/multipart_image.html")
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
pub fn load_image(multipart: Multipart, counter: Data<Cell<u32>>) -> Result<HttpResponse, Error> {
    println!("load image process initiated");

    //actually upload it to the server
    upload(multipart, counter);

    multipart_image()
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
    // pub fn save_file(field: Field, count: u32) -> impl Future<Item = i64, Error = Error> {
    let base = "upload_".to_owned();
    dbg!("wtf");
    println!("wtf is happened?");
    let code = match SystemTime::now().elapsed() {
        Ok(now) => {
            println!("now {}", now.as_secs());
            0
        }
        Err(why) => {
            println!("why {}", why);
            0
        }
    };
    // let file_path_string = base + ".png";
    let file_path_string = base + &code.to_string() + ".png";
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
    counter: Data<Cell<u32>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    counter.set(counter.get() + 1);
    println!("upload count {:?}", counter.get());


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