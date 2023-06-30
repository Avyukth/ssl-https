use std::{
    fs::File,
    io::{Read as _}
};

use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse,  HttpServer};
use  openssl::{
    pkey::{PKey, Private},
    ssl::{SslAcceptor, SslMethod},
};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
        .set_private_key(&load_encrypted_private_key())
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    log::info!("Listening on https://0.0.0.0:8080");
    HttpServer::new(|| {
        App::new()
        .wrap(middleware::Logger::default())
        .service(web::resource("/").route(web::get().to(index)))

    })
    .bind_openssl("0.0.0.0:8080", builder)?
    .workers(2)
    .run()
    .await
}

async fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
    // println!("{req:?}");
    Ok(HttpResponse::Ok()
    .content_type("text/plain; charset=utf-8")
    .body("Hello World TO hTTPS!"))
}

fn load_encrypted_private_key() ->  PKey<Private>{
    let mut file = File::open("key.pem").unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Failed to Read file");
    PKey::private_key_from_pem_passphrase(&buf, b"password").unwrap()
}
