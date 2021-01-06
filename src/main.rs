use actix_web::{get, App, HttpRequest, HttpServer, Responder};

#[get("/ping")]
async fn ping(_req: HttpRequest) -> impl Responder {
    "hello world\n"
}

#[get("/download")]
async fn download(_req: HttpRequest) -> impl Responder {
    "download\n"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(download)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}