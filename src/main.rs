use actix_web::{App, HttpServer};
mod routes;
use crate::routes::developers::{get_developers, post_developers};
use crate::routes::question::get_questions::get_questions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_developers)
            .service(post_developers)
            .service(get_questions)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
