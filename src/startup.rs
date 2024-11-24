use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslMethod};
use sqlx::PgPool;
use crate::route::echo;

pub fn run(address: String, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", openssl::ssl::SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/echo", web::get().to(echo))
            .app_data(web::PayloadConfig::new(2 * 1024 * 1024))
            .app_data(db_pool.clone())
            .app_data(web::Data::<String>::new("Ashura".to_owned()))
    })
    .bind_openssl(address, builder)?
    .run();
    Ok(server)
}
