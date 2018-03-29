use actix_web::*;
use futures::*;

const DEVICE_URI: &'static str = "https://www.rust-lang.org/en-US/";

pub fn index(_req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    // Try and send a get request to Kontakt API devices list, return count
    client::ClientRequest::get(DEVICE_URI)
        .finish().unwrap()
        .send()
        .map_err(error::Error::from)
        .and_then(
            |resp| resp.body()
                .from_err()
                .and_then(|body| {
                    httpcodes::HttpOk.build()
                        .body(body)
                        .map_err(error::Error::from)
                }))
        .responder()
}
