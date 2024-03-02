#![allow(non_snake_case)]

mod pages;
mod types;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::LevelFilter;
use pages::{QuizQuestion, Quizzes};

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    
    #[route("/")]
    Quizzes {},
    
    #[route("/quiz/:quiz_id/:question_id")]
    QuizQuestion { quiz_id: i32, question_id: i32 },
    
}

pub fn App(cx: Scope) -> Element {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    render! {
        Router::<Route> {}
    }
}

fn main() {
    dioxus_web::launch(App);
}