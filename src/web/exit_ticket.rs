use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
};

use crate::web::HtmlTemplate;
use askama::Template;
use axum::{Form, Router, response::IntoResponse, routing::get};

static FILE_PATH: &str = "./exit.csv";

struct Question {
    text: String,
}

#[derive(Template)]
#[template(path = "exit_ticket.html")]
struct ExitTicketTemplate {
    questions: Vec<Question>,
}

#[derive(Template)]
#[template(path = "done.html")]
struct DoneTemplate {}

pub fn router() -> Router {
    Router::new().route("/", get(self::get::root).post(self::post::submit))
}

mod get {
    use super::*;
    pub async fn root() -> impl IntoResponse {
        //let contents =
        //  fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
        //let data = json::parse(&contents).unwrap();

        let mut questions: Vec<Question> = vec![];

        let mut reader = csv::Reader::from_path(FILE_PATH).unwrap();

        for question in reader.headers().unwrap() {
            questions.push(Question {
                text: question.to_string(),
            });
        }

        HtmlTemplate(ExitTicketTemplate { questions })
    }
}

mod post {

    use super::*;
    pub async fn submit(Form(values): Form<HashMap<String, String>>) -> impl IntoResponse {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(FILE_PATH)
            .unwrap();
        let mut wtr = csv::Writer::from_writer(file);

        wtr.write_record(values.values()).unwrap();

        HtmlTemplate(DoneTemplate {})
    }
}
