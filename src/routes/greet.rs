use actix_web::{HttpRequest, Responder};

pub async fn greet(req: HttpRequest) -> impl Responder {
    // .query() will return a &str
    // .get() will return an Option as name may or may not exist in which case set a default value
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

