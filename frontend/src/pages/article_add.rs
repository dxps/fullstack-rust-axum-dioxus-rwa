use dioxus::{
    events::{FormData, MouseEvent},
    prelude::*,
};

use crate::comps::{FormButton_Lg, FormInput_Lg, FormTextarea_Lg};

pub fn ArticleAdd(cx: Scope) -> Element {
    let title = use_state(&cx, String::new);
    let summary = use_state(&cx, String::new);
    let content = use_state(&cx, String::new);
    let tags = use_state(&cx, String::new);
    cx.render(rsx! {
        div {
            class: "editor-page",
            div {
                class: "container page",
                h1 {
                    class: "text-xs-center",
                    "New Article"
                }
                br {}
                br {}

                div {
                    class: "row",

                    div {
                        class: "col-md-10 offset-md-1 col-xs-12",
                        form {
                            FormInput_Lg {
                                oninput: move |s: FormData| title.set(s.value),
                                placeholder: "Article Title".to_string()
                            }
                            FormInput_Lg {
                                oninput: move |s: FormData| summary.set(s.value),
                                placeholder: "What's this article about?".to_string()
                            }
                            FormTextarea_Lg {
                                oninput: move |s: FormData| content.set(s.value),
                                placeholder: "Article Title".to_string(),
                                rows: 8
                            }
                            FormInput_Lg {
                                oninput: move |s: FormData| tags.set(s.value),
                                placeholder: "Enter tags".to_string(),

                            }
                            div {
                                class: "tag-list"
                            }
                            FormButton_Lg {
                                onclick: move |_: MouseEvent| {
                                    log::info!(":: ArticleAdd] button clicked. title: {}", title);
                                    // TODO: Call the corresponding (HTTP) API operation, and all the rest.
                                },
                                label: "Publish Article".to_string()
                            }
                        }
                    }
                }
            }
        }
    })
}
