use chrono::prelude::*;
use rocket::{delete, get, post, serde::json::Json};
use uuid::Uuid;

use crate::models::*;

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(question: Json<Question>) -> Json<QuestionDetail> {
    let qd = QuestionDetail {
        title: question.title.clone(),
        description: question.description.clone(),
        question_uuid: Uuid::new_v4().to_string(),
        created_at: Utc::now().to_string(),
    };

    Json(qd)
}

#[get("/questions")]
pub async fn read_questions() -> Json<Vec<QuestionDetail>> {
    todo!()
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(question_uuid: Json<QuestionId>) {
    todo!()
}

// ---- CRUD for Answers ----

#[post("/answer", data = "<answer>")]
pub async fn create_answer(answer: Json<Answer>) -> Json<AnswerDetail> {
    todo!()
}

#[get("/answers")]
pub async fn read_answers() -> Json<Vec<AnswerDetail>> {
    todo!()
}

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(answer_uuid: Json<AnswerId>) {
    todo!()
}
