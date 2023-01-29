use super::types::{AuthRequest, QuestionItem, ThemeType};
use crate::routes::question::types::FileType;
use actix_web::web::Query;

pub fn check_file_type(file_type: &FileType) -> &'static str {
    match file_type {
        FileType::Json => "json",
        FileType::Csv => "csv",
        FileType::Xml => "xml",
        FileType::Yaml => "yaml",
    }
}

pub fn filter_questions(
    questions: Vec<QuestionItem>,
    info: Query<AuthRequest>,
    file_type: String,
) -> Vec<QuestionItem> {
    let mut filtered_questions: Vec<QuestionItem> = vec![];

    for item in questions.into_iter() {
        if (info.theme == ThemeType::All || info.theme == item.theme)
            && item.formats.contains(&file_type)
        {
            filtered_questions.push(item);
        }
    }
    filtered_questions
}
