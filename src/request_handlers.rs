use actix_web::*;
use futures::*;

const DEVICE_URI: &'static str = "https://www.rust-lang.org/en-US/";

pub fn index(_req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    // build the http request
    let request = client::ClientRequest::get(DEVICE_URI)
        .finish()
        .unwrap();

    // 
    let request = request.send().map_err(Error::from);

    let request_handler = request
        .and_then(
            |resp| resp.body()
                .from_err()
                .and_then(|body| {
                    Ok(HttpResponse::Ok().body(body))
                }));

    request_handler.responder()
}
