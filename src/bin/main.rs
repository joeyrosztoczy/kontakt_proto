extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate kontakt_proto;

use actix_web::*;
use kontakt_proto::request_handlers::get_devices;

const ADDRESS: &'static str = "127.0.0.1:8080";

// TODO: Figure out why SSL isn't working

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("http-proxy");

    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.f(get_devices))
    }).bind(ADDRESS)
        .expect("Why the fux")
        .run();

    let _sys = sys.run();
}
