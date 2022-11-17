use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use std::env;

pub async fn connect_db() {
    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(dotenv!("POSTGRES_URI")).await;
}
