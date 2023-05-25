use std::collections::HashMap;

use common_model::SuccessfulLoginDTO;
use dioxus::{
    events::{FormData, MouseEvent},
    prelude::*,
};
use dioxus_router::Link;
use reqwest::header::CONTENT_TYPE;

use crate::comps::{FormButton_Lg, FormInput_Lg};

pub fn SignInPage(cx: Scope) -> Element {
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
                            "Sign in"
                        }
                        p {
                            class: "text-xs-center",
                            Link { to: "/signup", "Don't have an account?" }
                        }
                        br {}
                        br {}

                        ul {
                            class: "error-messages",
                            li { "Invalid credentials" }
                        }

                        form {
                            FormInput_Lg {
                                oninput: move |s: FormData| email.set(s.value),
                                placeholder: "Email".to_string()
                            }
                            FormInput_Lg {
                                oninput: move |s: FormData| password.set(s.value),
                                placeholder: "Password".to_string()
                            }
                            FormButton_Lg {
                                onclick: |_: MouseEvent| {
                                    log::info!("[SignInPage] button clicked. email: {}", email.get());
                                    // TODO: Call the corresponding (HTTP) API operation, and all the rest.
                                    let email = email.get().clone();
                                    let password = password.get().clone();
                                    cx.use_hook(|| crate::block_on(login(email, password)));
                                },
                                label: "Sign in".to_string()
                            }
                        }
                    }
                }
            }
        }
    })
}

async fn login(email: String, password: String) {
    let mut req_creds = HashMap::new();
    req_creds.insert("email", email);
    req_creds.insert("password", password);
    let mut req_body = HashMap::new();
    req_body.insert("user", req_creds);

    match reqwest::Client::new()
        .post("http://localhost:8001/api/users/login")
        .header(CONTENT_TYPE, "application/json")
        .json(&req_body)
        .send()
        .await
    {
        Ok(res) => {
            let token = res.json::<SuccessfulLoginDTO>().await.unwrap().user.token;
            log::info!("[login] Got token {}", token.unwrap())
            // TODO: set it in the state
        }
        Err(_err) => {
            // log
        }
    }
}
