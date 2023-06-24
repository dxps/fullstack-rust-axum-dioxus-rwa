use dioxus::prelude::*;

pub fn NotFoundPage(cx: Scope) -> Element {
    //
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
