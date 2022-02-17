use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/{id}")]
async fn index(id: web::Path<i32>) -> impl Responder {
  let id = id.into_inner();
  format!("Hello {}!", id)
}

#[post("/text")]
async fn text(payload: String) -> impl Responder {
  HttpResponse::Ok().body(payload)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(index)
      .service(text)
  })
  .bind("0.0.0.0:8089")?
  .run()
  .await
}
