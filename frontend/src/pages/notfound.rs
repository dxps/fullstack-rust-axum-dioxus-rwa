use dioxus::{
    events::{FormData, MouseEvent},
    prelude::*,
};
use dioxus_router::Link;

use crate::comps::{FormButton_Lg, FormInput_Lg};

pub fn NotFoundPage(cx: Scope) -> Element {
    let email = use_state(&cx, String::new);
    let password = use_state(&cx, String::new);

    cx.render(rsx! {
        div {
            class: "auth-page",
            div {
                class: "container page",
                div {
                    class: "row",
                    div {
                        class: "col-md-6 offset-md-3 col-xs-12",
                        h1 {
                            class: "text-xs-center",
                            "Oops! Unknown page"
                        }
                        p {
                            class: "text-xs-center",
                            "The page you are looking for doesn't exist!"
                        }
                        br {}
                    }
                }
            }
        }
    })
}
