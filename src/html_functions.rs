use actix_web::{HttpResponse, Responder, web};
use std::sync::Arc;
use tera::{Context, Tera};

pub async fn home_page(tera: web::Data<Arc<Tera>>) -> impl Responder {
    let mut context: Context = Context::new();
    context.insert("name", "TERA");
    let page_rendered: String = tera
        .render("01_basic_layout.html", &context)
        .expect("Unable to render the page");
    HttpResponse::Ok().body(page_rendered)
}

pub async fn live_server(tera: web::Data<Arc<Tera>>) -> impl Responder {
    let mut context: Context = Context::new();
    context.insert("title", "Live Actix");

    let rendered = tera
        .render("02_live_server.html", &mut context)
        .expect("unable to render");
    HttpResponse::Ok().body(rendered)
}
