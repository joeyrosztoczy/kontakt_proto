extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate kontakt_proto;

use actix_web::*;
use kontakt_proto::request_handlers::index;

const ADDRESS: &'static str = "127.0.0.1:8080";

// TODO: Figure out why SSL isn't working

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("http-proxy");

    HttpServer::new(|| {
        Application::new()
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.f(index))
    }).bind(ADDRESS)
        .expect("Why the fux")
        .run();

    sys.run();

    println!("Running the kontakt prototype at: {}", ADDRESS);
}
