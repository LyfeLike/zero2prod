use std::net::TcpListener;

use zero2prod::run;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    run(listener)?.await
}
//page 40 in pdf
