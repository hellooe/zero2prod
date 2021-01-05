use dotenv::{dotenv, var};
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::telemetry::init_subscriber;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_subscriber();
    dotenv().ok();
    let db_url = var("DATABASE_URL").unwrap();
    let db_pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to Postgres.");
    let addr = var("ADDRESS").unwrap();
    let listener = TcpListener::bind(addr)?;
    run(listener, db_pool).await
}
