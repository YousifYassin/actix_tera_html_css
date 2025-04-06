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

pub async fn meta_tags(tera: web::Data<Arc<Tera>>) -> impl Responder {
    let mut context: Context = Context::new();
    context.insert("title", "Meta Tags");
    let rendered = tera
        .render("03_meta_tags.html", &context)
        .expect("unable to render the page");
    HttpResponse::Ok().body(rendered)
}

pub async fn typography(tera: web::Data<Arc<Tera>>) -> impl Responder {
    let mut context: Context = Context::new();
    let the_range: Vec<i32> = (1..=6).into_iter().collect();
    context.insert("title", "Headings, Paragraph, Typography");
    context.insert("numbers", &the_range);
    let rendered = tera
        .render("04_typography.html", &context)
        .expect("Unable to render the page");
    HttpResponse::Ok().body(rendered)
}
