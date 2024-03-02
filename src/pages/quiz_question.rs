use dioxus::prelude::*;

use crate::types::{get_quizzes, QuestionOption};

#[component]
pub fn QuizQuestion(cx: Scope, quiz_id: i32, question_id: i32) -> Element {
    let quizzes = get_quizzes();
    let quiz = quizzes.into_iter().find(|q| &q.id == quiz_id).unwrap();
    let question = quiz
        .questions
        .into_iter()
        .find(|q| &q.id == question_id)
        .unwrap();

    let curr_answer = use_state(cx, || Option::<QuestionOption>::None);
    cx.render(rsx! {
        div { class: "flex flex-col min-h-screen bg-blue-200 w-full items-center",
            div { class: "h-10"},
            div { class: "flex flex-col w-3/4 p-4 border-2 border-black rounded-xl bg-white shadow-xl gap-4",
                div { class: "text-4xl font-bold", quiz.title},
                div { class: "text-2xl font-bold", question.title},
                div { class: "text-gray text-lg", question.description},
                div { class: "flex flex-wrap gap-4",
                    question.options.into_iter().map(|option| {
                        let this_option = option.clone();
                        let button_color = match &curr_answer.get() {
                            Some(answer) if answer.id == option.id => "bg-blue-500",
                            _ => "bg-blue-300",
                        };
                        let button_class = format!("flex flex-row gap-4 p-4 rounded-full shadow-xl {}", button_color);
                        return rsx! {
                            button {
                                class: "{button_class}",
                                onclick: move |_| curr_answer.set(Some(this_option.clone())),
                                div { class: "text-lg", option.title },
                            }
                        }
                    }),
                },
            },
        }
    })
}
