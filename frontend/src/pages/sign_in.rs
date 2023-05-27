use std::collections::HashMap;

use common_model::SuccessfulLoginDTO;
use dioxus::{
    events::{FormData, MouseEvent},
    prelude::*,
};
use dioxus_router::{use_router, Link};
use reqwest::header::CONTENT_TYPE;

use crate::comps::{FormButton_Lg, FormInput_Lg};

pub fn SignInPage(cx: Scope) -> Element {
    let email = use_state(&cx, String::new);
    let password = use_state(&cx, String::new);
    let hide_invalid_creds = use_state(cx, String::new);
    let router = use_router(cx);

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
                            hidden: hide_invalid_creds.get().as_str(),
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
                                    let hide_invalid_creds = hide_invalid_creds.clone();
                                    let router = router.clone();
                                    cx.spawn({
                                        async move {
                                            match login(email, password).await {
                                                true => {
                                                    log::info!("[SignInPage] successful login");
                                                    router.push_route("/", None, None);
                                                },
                                                false => {
                                                    log::info!("[SignInPage] failed login");
                                                    // Show 'Invalid credentials'
                                                    hide_invalid_creds.set("false".into());
                                                }
                                            }
                                        }
                                    });
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
                    log::debug!("[login] Invalid credentials");
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
