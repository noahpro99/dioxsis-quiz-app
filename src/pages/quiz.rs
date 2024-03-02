use dioxus::prelude::*;

use crate::types::{self, get_quizzes, Question, QuestionOption};

async fn get_quiz(quiz_id: i32) -> types::Quiz {
    get_quizzes().into_iter().find(|q| q.id == quiz_id).unwrap()
}

#[component]
pub fn Quiz(cx: Scope, quiz_id: i32) -> Element {
    let answers = use_state(cx, || Vec::<QuestionOption>::new());
    let can_move_on = use_state(cx, || false);
    let ques_id = use_state(cx, || 0);

    let question = use_future!(cx, |(quiz_id, ques_id)| async move {
        let quiz = get_quiz(quiz_id).await;
        log::debug!("Got quiz: {:?}", quiz);
        quiz.questions.get(*ques_id.get() as usize).cloned()
    });

    cx.render(rsx! {
        div { class: "flex flex-col min-h-screen bg-blue-200 w-full items-center",
            div { class: "h-10"},
            match question.value() {
                Some(Some(question)) => rsx! { QuizQuestion {question: question} },
                _ => rsx! { div { class: "text-2xl font-bold", "Loading..."} },
            },
        }
    })
}

#[derive(Props)]
struct QuizQuestionProps<'a> {
    question: &'a Question,
}

fn QuizQuestion<'a>(cx: Scope<'a, QuizQuestionProps<'a>>) -> Element<'a> {
    let Question {
        id,
        title,
        description,
        options,
    } = cx.props.question;
    let curr_answer = use_state(cx, || Option::<QuestionOption>::None);
    cx.render(rsx! {
        div { class: "flex flex-col w-3/4 p-4 border-2 border-black rounded-xl bg-white shadow-xl gap-4",
            div { class: "text-2xl font-bold", "{title}"},
            div { class: "text-gray text-lg", "{description}"},
            div { class: "flex flex-wrap gap-4",
                options.into_iter().map(|option| {
                    let this_option = option.clone();
                    let button_color = match &curr_answer.get() {
                        Some(answer) if answer.id == option.id => "bg-blue-500",
                        _ => "bg-blue-300",
                    };
                    return rsx! {
                        button {
                            class: "flex flex-row gap-4 p-4 rounded-full shadow-xl {button_color}",
                            onclick: move |_| curr_answer.set(Some(this_option.clone())),
                            div { class: "text-lg", "{option.title}" },
                        }
                    }
                }),
            },
        }
    })
}
