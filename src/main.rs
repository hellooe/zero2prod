use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let lst_addr = std::env::var("LST_ADDR").unwrap_or_else(|_| "127.0.0.1:8000".into());
    let db_url = std::env::var("DATABASE_URL").unwrap();

    run(&lst_addr, &db_url).await
}
