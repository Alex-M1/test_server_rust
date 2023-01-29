use actix_web::{get, post, HttpResponse, Responder};
use std::{fs, path::Path};

const PATH: &str = "./data/developers.json";

#[get("/team")]
pub async fn get_developers() -> impl Responder {
    let data = fs::read_to_string(Path::new(PATH)).expect("Cannot read file");
    HttpResponse::Ok().body(data)
}

#[post("/team")]
pub async fn post_developers(req_body: String) -> impl Responder {
    let data = fs::write(PATH, &req_body);
    if data.is_err() {
        return HttpResponse::ExpectationFailed().body("Something went wrong");
    }
    HttpResponse::Ok().body(req_body)
}
