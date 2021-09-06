extern crate actix_web;
extern crate openssl;

use actix_web::{web, HttpServer, App};

use webapp::routes::{hello, echo, add_user};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    
    let app = move || { App::new()
        .service(hello)
        .service(echo)
        .route("add", web::get().to(add_user))

    };


    let host = "127.0.0.1:8010";
    
    
    HttpServer::new(app)
    .bind_openssl(host, builder)?
    .run()
    .await
}
