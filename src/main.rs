use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod db;
mod errors;
mod services;
mod jresult;

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("Merhaba Dünya")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Starting HTTP server at http://localhost:3001");

    let pool = db::get_pool().await;

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(pool.clone()))
            .service(actix_files::Files::new("/public", "public").show_files_listing())
            .service(actix_files::Files::new("/static", "public/static").show_files_listing())
            .route("/", web::get().to(index))
            .configure(services::init)
    })
    .bind("0.0.0.0:3001")?
    .run()
    .await
}
