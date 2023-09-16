#[macro_use]
extern crate rocket;

mod cors;
mod handlers;
mod models;
use models::*;

use cors::*;
use handlers::*;
use log::info;
use sqlx::{postgres::PgPoolOptions, Row};

#[launch]
async fn rocket() -> _ {
    // Initialize pretty_env_logger
    pretty_env_logger::init();
    // Initialize dotenv
    dotenvy::dotenv().ok();
    let db_url = dotenvy::var("DATABASE_URL").expect("could not read 'DATABASE_URL'");

    // Create a new PgPoolOptions instance with a maximum of 5 connections.
    // See examples on GitHub page: https://github.com/launchbadge/sqlx
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("could not connect to DB");

    // Using slqx, execute a SQL query that selects all questions from the questions table.
    let recs = sqlx::query("SELECT * FROM questions;")
        .fetch_all(&pool)
        .await
        .expect("could not query DB");

    info!("********* Question Records *********");
    // Log recs with debug formatting using the info! macro
    let results = recs
        .iter()
        .map(|r| Question {
            description: r.get("description"),
            title: r.get("title"),
        })
        .collect::<Vec<Question>>();
    results.iter().for_each(|q| info!("{q:?}"));

    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                read_questions,
                delete_question,
                create_answer,
                read_answers,
                delete_answer
            ],
        )
        .attach(CORS)
}
