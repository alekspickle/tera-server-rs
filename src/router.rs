use actix_web::{error, Error, HttpResponse, State};

pub struct AppState {
    template: tera::Tera,
}

pub fn index(state: State<AppState>) -> Result<HttpResponse, Error> {
    let mut _ctx = tera::Context::new();
    render_page(state, "pages/index.html")
}

pub fn detail(state: State<AppState>) -> Result<HttpResponse, Error> {
    let mut _ctx = tera::Context::new();

    render_page(state, "pages/detail.html")
}
pub fn calculate(state: State<AppState>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let calculated: u32 = 10 + 9876;
    ctx.insert("calculated", &calculated.to_owned());
    render_with_ctx(state, "pages/calculate.html", ctx)
}

pub fn p404(state: State<AppState>) -> Result<HttpResponse, Error> {
    let mut _ctx = tera::Context::new();
    render_page(state, "pages/404.html")
}

pub fn render_with_ctx(
    state: State<AppState>,
    template: &str,
    ctx: tera::Context,
) -> Result<HttpResponse, Error> {
    let s = state
        .template
        .render(template, &ctx.to_owned())
        .map_err(|e| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub fn render_page(state: State<AppState>, template: &str) -> Result<HttpResponse, Error> {
    let s = state
        .template
        .render(template, &tera::Context::new())
        .map_err(|e| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
