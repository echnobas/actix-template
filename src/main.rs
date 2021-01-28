mod errors;
use errors::*;

use actix_web::{
    get,
    web,
    App,
    HttpRequest,
    HttpResponse,
    HttpServer,
    middleware::{
        Logger,
        NormalizePath,
        normalize::TrailingSlash
    },
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(Logger::default())
            .default_service(web::route().to(p404))
            .configure(init_api)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn init_api(cfg: &mut web::ServiceConfig) {
    cfg.service(
    web::scope("/api/v1").service(index)
            .default_service(web::route().to(p404))
    );
}

#[get("")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().json(r#"{"status": "running"}"#)
}

async fn p404(_: HttpRequest) -> Result<HttpResponse, ApplicationError> {
    Err(ApplicationError::NotFound)
}
