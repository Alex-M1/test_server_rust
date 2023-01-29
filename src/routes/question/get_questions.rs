use super::helpers::check_file_type;
use super::helpers::filter_questions;
use super::parsers::{csv_to_json, json_str_to_json};
use super::types::{AuthRequest, ErrorMessage, QuestionItem, PATH};
use actix_web::{get, web::Query, HttpResponse, Responder};
use std::fs;

#[get("/questions")]
pub async fn get_questions(info: Query<AuthRequest>) -> impl Responder {
    let file_type = check_file_type(&info.file).to_string();
    let formated_path = format!("{}.{}", PATH, file_type.to_string());
    let questions_data = fs::read_to_string(formated_path);

    let questions = match questions_data {
        Ok(value) => value,
        Err(e) => {
            let error_message = ErrorMessage {
                message: e.to_string(),
            };
            return HttpResponse::InternalServerError().json(error_message);
        }
    };
    let questions_json: Vec<QuestionItem> = json_str_to_json(&questions);
    csv_to_json(&format!("{}.{}", PATH, file_type.to_string()));

    let filtered_questions = filter_questions(questions_json, info, file_type);

    HttpResponse::Ok().json(filtered_questions)
}
