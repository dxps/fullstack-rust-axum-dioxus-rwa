use std::collections::HashMap;

use common_model::SuccessfulLoginDTO;
use dioxus::{
    events::{FormData, MouseEvent},
    prelude::*,
};
use dioxus_router::{Link, Redirect};
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
                                    let email = email.get().clone();
                                    let password = password.get().clone();
                                    if let Some(res) = use_future!(cx, |()| async move {
                                        login(email, password).await
                                    }).value() {
                                        match res {
                                            true => {
                                                log::info!("[SignInPage] successful login");
                                                cx.render(rsx! { Redirect { to: "/"} });
                                            },
                                            false => {
                                                log::info!("[SignInPage] failed login");
                                            }
                                        }
                                    }
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

async fn login(email: String, password: String) -> bool {
    let mut req_creds = HashMap::new();
    req_creds.insert("email", email);
    req_creds.insert("password", password);
    let mut req_body = HashMap::new();
    req_body.insert("user", req_creds);

    match reqwest::Client::new()
        .post("http://localhost:8081/api/users/login")
        .header(CONTENT_TYPE, "application/json")
        .json(&req_body)
        .send()
        .await
    {
        Ok(res) => {
            match res.status().as_u16() {
                200 => {
                    match res.json::<SuccessfulLoginDTO>().await {
                        Ok(dto) => {
                            log::debug!("[login] Successful login: {:#?}", dto);
                            // TODO: set it in the state
                        }
                        Err(e) => log::debug!("[login] Failed to deserialize response: {}", e),
                    }
                    true
                }
                401 => {
                    log::debug!("[login] Wrong credentials");
                    false
                }
                _ => {
                    log::debug!(
                        "[login] Unexpected login response status: {} body: {}",
                        res.status(),
                        res.text().await.unwrap_or_default()
                    );
                    false
                }
            }
        }
        Err(e) => {
            log::error!("[login] Request failed: {}", e);
            false
        }
    }
}
