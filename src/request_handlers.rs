use actix_web::*;
use futures::*;
use std::env;

const KONTAKT_URI: &'static str = "https://api.kontakt.io/";

pub fn index(_req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    // build the http request
    let request = client::ClientRequest::get(KONTAKT_URI)
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

pub fn get_devices(_req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let kontakt_api_key = env::var("KONTAKT_API_KEY").unwrap();
    // Build an http request to get a list of devices from kontakt api
    let request = client::ClientRequest::get(KONTAKT_URI.to_owned() + "device?deviceType=beacon")
        .header("Api-key", kontakt_api_key)
        .header("Accept", "application/vnd.com.kontakt+json; version=10")
        .finish()
        .unwrap();

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
