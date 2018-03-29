use actix_web::HttpRequest;

pub fn index(req: HttpRequest) -> &'static str {
    "You made it to me werker"
}
