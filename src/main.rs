use actix_tera_html_css::html_functions::{home_page, live_server, meta_tags};
use actix_web::{
    App, HttpServer,
    web::{self, get},
};
use std::{io, sync::Arc};
use tera::Tera;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let tera: Arc<Tera> =
        Arc::new(Tera::new("templates/html_section/**/*").expect("Unable to start Tera"));
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&tera)))
            .route("/", get().to(home_page))
            .route("/live_server", get().to(live_server))
            .route("/meta_tags", get().to(meta_tags))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
