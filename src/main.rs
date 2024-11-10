use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod db {
    pub mod config;
}

#[derive(Clone)]
pub struct AppState {
    postgres_client: Pool<Postgres>,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    dotenv().ok();
    let _pool = db::config::start_connection().await;
    HttpServer::new(move || {
        App::new()
            .app_data(
                web::Data::new(AppState {
                    postgres_client: _pool.clone(),
                })
            )
            .service(index)
    }).bind(("127.0.0.1", 8080))?.run().await
}
