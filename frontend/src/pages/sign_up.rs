use dioxus::{
    events::{FormData, MouseEvent},
    prelude::*,
};

use crate::comps::{FormButton_Lg, FormField_Lg};

pub fn SignUp(cx: Scope) -> Element {
    //
    let name = use_state(&cx, String::new);
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
                            "Sign up"
                        }
                        p {
                            class: "text-xs-center",
                            Link { to: "/signin", "Have an account?" }
                        }

                        ul {
                            class: "error-messages",
                            li { "That email is already taken" }
                        }

                        form {
                            FormField_Lg {
                                oninput: move |s: FormData| {
                                    name.set(s.value);
                                },
                                placeholder: "Your Name".to_string()
                            }
                            FormField_Lg {
                                oninput: move |s: FormData| email.set(s.value),
                                placeholder: "Email".to_string()
                            }
                            FormField_Lg {
                                oninput: move |s: FormData| password.set(s.value),
                                placeholder: "Password".to_string()
                            }
                            FormButton_Lg {
                                onclick: move |_: MouseEvent| {
                                    log::info!("[SignUp] button clicked. name: {} | email: {}", name, email);
                                    // TODO: Call the signup (HTTP) API operation, and all the rest. 
                                },
                                label: "Sign up".to_string()
                            }
                        }
                    }
                }
            }
        }
    })
}
