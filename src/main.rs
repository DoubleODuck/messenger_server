use env_logger::Env;
use sqlx::PgPool;
use test_server::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()>{
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let connection_pool = PgPool::connect_lazy("postgresql://postgres:postgres@localhost/")
        .expect("Failed to connect to Postgres.");
    let address = format!("0.0.0.0:{}", 8443);
    log::info!("Connected");
    run(address,connection_pool)?.await
}