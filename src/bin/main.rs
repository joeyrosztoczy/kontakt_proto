extern crate actix_web;
extern crate kontakt_proto;

use actix_web::*;
use kontakt_proto::request_handlers::index;

const ADDRESS: &'static str  = "127.0.0.1:8080";

fn main() {
    println!("Running the kontakt prototype at: {}", ADDRESS);

    HttpServer::new(|| Application::new().resource("/", |r| r.f(index)))
        .bind(ADDRESS)
        .expect("Why the fux")
        .run();
}
