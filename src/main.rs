#![warn(
    clippy::all,
    clippy::pedantic,
)]
#![allow(clippy::implicit_return)]

mod errors;
mod models;

#[macro_use] extern crate bson;
use errors::ApplicationError;
use actix_web::{
    get,
    post,
    web::{
        self,
        Json
    },
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

const DATABASE_URI: &str = "mongodb+srv://administrator:root@cluster0.kobuv.mongodb.net/<dbname>?retryWrites=true&w=majority";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = mongodb::Client::with_uri_str(DATABASE_URI).await.unwrap();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .data(client.clone())
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
    web::scope("/api/v1")
        .service(index)
        .service(new_user)
        .default_service(web::route().to(p404))
    );
}

#[get("")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().json(r#"{"status": "running"}"#)
}

#[post("/new")]
async fn new_user(user: Json<models::User>, client: web::Data<mongodb::Client>) -> Result<HttpResponse, ApplicationError> {
    let collection = client.database("rust_api").collection("users");
    collection.insert_one(bson::Document::from(user.into_inner()), None).await?;
    Err(ApplicationError::Internal)
}

async fn p404(_: HttpRequest) -> Result<HttpResponse, ApplicationError> {
    Err(ApplicationError::NotFound)
}
