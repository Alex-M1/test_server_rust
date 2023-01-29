use super::types::QuestionItem;
use csv::Reader;
use serde_json::from_str;

pub fn json_str_to_json(questions: &String) -> Vec<QuestionItem> {
    let questions_json: Vec<QuestionItem> = from_str(&questions).expect("Something went wrong");
    questions_json
}

pub fn csv_to_json(questions: &String) {
    let mut rdr = Reader::from_path("./data/questions.csv").unwrap();
    for result in rdr.records() {
        let record = result.unwrap();
        println!("{:?}", record);
    }
}
