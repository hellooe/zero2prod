use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::configuration::get_config;
use zero2prod::startup::run;
use zero2prod::telemetry::init_subscriber;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_subscriber();
    let config = get_config().expect("Failed to read configuration.");
    let db_pool = PgPoolOptions::new()
        .connect_with(config.database.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    let addr = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(addr)?;
    run(listener, db_pool).await
}
