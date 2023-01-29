use serde::{Deserialize, Serialize};

pub const PATH: &str = "./data/questions";

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub file: FileType,
    pub theme: ThemeType,
}

#[derive(Debug, Deserialize)]
pub enum FileType {
    Json,
    Csv,
    Xml,
    Yaml,
}
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum ThemeType {
    All,
    Html,
    Css,
    Js,
    React,
    Oop,
}
#[derive(Serialize)]
pub struct ErrorMessage {
    pub message: String,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct QuestionItem {
    pub id: i32,
    pub question: String,
    pub answer: bool,
    pub theme: ThemeType,
    pub formats: Vec<String>,
}
