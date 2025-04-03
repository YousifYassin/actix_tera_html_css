use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::sync::Arc;
use tera::{Context, Tera};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera: Arc<Tera> = Arc::new(Tera::new("templates/**/*").expect("Failed to start Tera"));
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&tera)))
            .route("/", web::get().to(home_page))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

async fn home_page(tera: web::Data<Arc<Tera>>) -> impl Responder {
    let context: Context = Context::new();
    let page = tera
        .render("index.html", &context)
        .expect("Failed to render");
    HttpResponse::Ok().body(page)
}
