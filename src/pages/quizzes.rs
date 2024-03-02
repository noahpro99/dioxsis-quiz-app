use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{types::get_quizzes, Route};

// create a component that renders a div with the text "Hello, world!"
#[component]
pub fn Quizzes(cx: Scope) -> Element {
    let quizzes = get_quizzes();

    cx.render(rsx! {
        div { class: "flex flex-col items-center pt-20 gap-2 bg-blue-200 min-h-screen",
            div { class: "text-4xl font-bold", "Quizzes"},
            div { class: "h-10"}
            quizzes.into_iter().map(|quiz| rsx! {
                Link { class: "bg-white shadow-xl rounded-xl p-4 w-3/4 border-2 border-black",
                    to: Route::Quiz { quiz_id: quiz.id },
                    h1 { class: "text-2xl font-bold",quiz.title},
                    p { class: "text-gray-600", quiz.description},
                }
            })
        }
    })
}
