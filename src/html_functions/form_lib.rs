use actix_web::{HttpResponse, Responder, get, post, web};
use serde::Deserialize;
use std::sync::Arc;
use tera::{Context, Tera};

#[derive(Deserialize, Debug)]
pub struct UserFormData {
    pub person_username: String,
    pub person_email: String,
    pub person_message: String,
    pub person_gender: String,
    pub person_likes_bike: Option<Likes>,
    pub person_likes_programming: Option<Likes>,
    pub person_likes_football: Option<Likes>,
}

#[derive(Deserialize, Debug)]
pub enum Likes {
    Bike,
    Programming,
    Football,
}

#[get("/forms_input")]
pub async fn forms_input(tera: web::Data<Arc<Tera>>) -> impl Responder {
    let mut context: Context = Context::new();
    context.insert("title", "Forms input");
    let page: String = tera
        .render("07_forms_input.html", &context)
        .expect("Unable to render the page content");
    HttpResponse::Ok().body(page)
}

#[post("/forms_input")]
pub async fn submit(tera: web::Data<Arc<Tera>>, form: web::Form<UserFormData>) -> impl Responder {
    println!("{:#?}", form);
    let mut context: Context = Context::new();
    context.insert("title", "Forms input");
    let page: String = tera
        .render("07_forms_input.html", &context)
        .expect("Unable to render the page content");
    HttpResponse::Ok().body(page)
}
