use crate::routes::{health_check, subscribe};
use actix_web::{middleware, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub async fn run(listener: TcpListener, db_pool: PgPool) -> std::io::Result<()> {
    let db_pool = web::Data::new(db_pool);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run()
    .await
}
