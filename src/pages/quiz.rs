use dioxus::prelude::*;
use dioxus_router::components::Link;

use crate::{
    types::{self, get_quizzes, Question, QuestionOption},
    Route,
};
use itertools::Itertools;

async fn get_quiz(quiz_id: i32) -> types::Quiz {
    get_quizzes().into_iter().find(|q| q.id == quiz_id).unwrap()
}

#[component]
pub fn Quiz(cx: Scope, quiz_id: i32) -> Element {
    let answers = use_ref(cx, || Vec::<QuestionOption>::new());
    let ques_id = use_state(cx, || 0);

    let question = use_future!(cx, |(quiz_id, ques_id)| async move {
        let quiz = get_quiz(quiz_id).await;
        quiz.questions.get(*ques_id.get() as usize).cloned()
    });

    let total_questions = use_future!(cx, |quiz_id| async move {
        let quiz = get_quiz(quiz_id).await;
        quiz.questions.len() as i32
    });

    cx.render(rsx! {
        div { class: "flex flex-col min-h-screen bg-blue-200 w-full items-center",
            div { class: "h-10" }
            div { class: "flex flex-col w-3/4 p-4 border-2 border-black rounded-xl bg-white shadow-xl gap-4",
                match question.value() {
                    Some(Some(question)) => rsx! { QuizQuestion {question: question, answers: answers, ques_id: ques_id} },
                    Some(None) => {
                        if let Some(total_questions) = total_questions.value() {
                            let score = (answers.read().iter().filter(|a| a.is_correct).collect_vec().len() as f32 / *total_questions as f32 * 100.0) as i32;
                            rsx! { ResultsPage {score: score} }
                        } else
                        {rsx! { ResultsPage {score: 0} }}
                    },
                    _ => rsx! { div { class: "text-2xl font-bold", "Loading..." } },
                }
            }
        }
    })
}

#[derive(Props)]
struct QuizQuestionProps<'a> {
    question: &'a Question,
    answers: &'a UseRef<Vec<QuestionOption>>,
    ques_id: &'a UseState<i32>,
}

fn QuizQuestion<'a>(cx: Scope<'a, QuizQuestionProps<'a>>) -> Element<'a> {
    let Question {
        id,
        title,
        description,
        options,
    } = cx.props.question;
    let answers = cx.props.answers;
    let ques_id = cx.props.ques_id;

    let curr_answer = use_state(cx, || Option::<QuestionOption>::None);

    cx.render(rsx! {
        div { class: "text-2xl font-bold", "{title}" }
        div { class: "text-gray text-lg", "{description}" }
        div { class: "flex flex-col gap-4 py-2",
            options.into_iter().map(|option| {
                let this_option = option.clone();
                let selected_button_classes = match &curr_answer.get() {
                    Some(answer) if answer.id == option.id => "bg-blue-500 border-black",
                    _ => "bg-blue-300",
                };
                rsx! {
                    button {
                        class: "flex flex-row gap-4 p-4 rounded-full shadow-xl {selected_button_classes} border-2 hover:-translate-y-1 hover:shadow-2xl",
                        onclick: move |_| curr_answer.set(Some(this_option.clone())),
                        div { class: "text-lg", "{option.title}" },
                    }
                }
            })
        }
        div { class: "flex flex-row justify-between w-full",
            button {
                class: "p-4 bg-blue-500 rounded-full shadow-xl hover:-translate-y-1",
                onclick: move |_| {
                    ques_id.set(ques_id.get() - 1);
                },
                "Back"
            }
            button {
                class: "p-4 bg-blue-500 rounded-full shadow-xl hover:-translate-y-1",
                onclick: move |_| {
                    if let Some(answer) = curr_answer.get() {
                        answers.write().push(answer.clone());
                        ques_id.set(ques_id.get() + 1);
                    }
                },
                "Next"
            }
        }
    })
}

#[component]
fn ResultsPage(cx: Scope, score: i32) -> Element {
    cx.render(rsx! {
        div { class: "flex flex-col min-h-32 w-full items-center",
            div { class: "h-8" }
            div { class: "text-2xl font-bold p-2", "You got {score}% correct!" }
            div { class: "text-lg p-2", "Thanks for taking the quiz!" }
            Link { class: "p-4 bg-blue-500 rounded-full shadow-xl hover:-translate-y-1",
                to: Route::Quizzes {},
                "Back to Home"
            }
        }
    })
}
