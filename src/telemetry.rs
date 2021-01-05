pub fn init_subscriber() {
    tracing_subscriber::fmt()
        .with_env_filter("zero2prod=info,actix_web=info")
        .init();
}
