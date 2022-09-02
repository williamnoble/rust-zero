use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    // HttpResponse::Ok() is part of the HttpResponseBuilder, the finish returns the HttpResponse.
    // If we instead returned impl Responder we can use HttpResponse::Ok().
    HttpResponse::Ok().finish()
}