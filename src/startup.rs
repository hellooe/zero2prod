use crate::routes::{health_check, subscribe};
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

async fn get_db_pool(db_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect(db_url)
        .await
}

pub async fn run(lst_addr: &str, db_url: &str) -> std::io::Result<()> {
    let db_pool = get_db_pool(db_url)
        .await
        .expect("Failed to connect to Postgres.");
    let db_pool = web::Data::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .bind(lst_addr)?
    .run()
    .await
}
