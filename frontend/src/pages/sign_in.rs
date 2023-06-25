use std::collections::HashMap;

use common_model::SuccessfulLoginDTO;
use dioxus::{
    events::{FormData, MouseEvent},
    prelude::*,
};
use dioxus_router::{use_router, Link};
use dioxus_use_storage::use_session_storage;
use reqwest::header::CONTENT_TYPE;

use crate::{
    commons::{AppState, TOKEN},
    comps::{FormButton_Lg, FormInput_Lg},
};

pub fn SignInPage(cx: Scope) -> Element {
    //
    let email = use_state(&cx, String::new);
    let password = use_state(&cx, String::new);
    let hide_invalid_creds = use_state(&cx, String::new);
    let hide_internal_err = use_state(&cx, String::new);
    let router = use_router(&cx);
    let session_storage = use_session_storage(cx);
    let token_state = use_state(&cx, String::new);
    let token = token_state.get();
    if token.len() > 0 {
        session_storage.insert(TOKEN, token);
        let app_state = use_shared_state::<AppState>(cx).unwrap();
        app_state.write().token = Some(token.clone());
        router.push_route("/", None, None);
    }

    // TODO: Temporary used during development.
    let temp_email = "joe@black.com".to_string();
    let temp_pass = "123".to_string();

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
                            li { hidden: hide_invalid_creds.get().as_str(), "Invalid credentials" }
                            li { hidden: hide_internal_err.get().as_str(), "Internal error during login. Please try again later." }
                        }

                        form {
                            FormInput_Lg {
                                oninput: move |s: FormData| email.set(s.value),
                                placeholder: "Email".to_string()
                                value: temp_email // TODO: Temporary used during development.
                            }
                            FormInput_Lg {
                                oninput: move |s: FormData| password.set(s.value),
                                placeholder: "Password".to_string()
                                value: temp_pass // TODO: Temporary used during development.
                            }
                            FormButton_Lg {
                                onclick: move |_: MouseEvent| {
                                    // TODO: Temporary used during development.
                                    // let email = email.get().clone();
                                    // let password = password.get().clone();
                                    let email = "joe@black.com".to_string();
                                    let password = "123".to_string();
                                    
                                    let hide_invalid_creds = hide_invalid_creds.clone();
                                    let hide_internal_err = hide_internal_err.clone();
                                    let token_state = token_state.clone();
                                    cx.spawn({
                                        async move {
                                            match login(email, password).await {
                                                Ok(token) => {
                                                    token_state.set(token);
                                                },
                                                Err(msg) => {
                                                    log::debug!(":: SignInPage :: Failed login.");
                                                    match msg.as_str() {
                                                        "invalid_credentials" => {
                                                            hide_invalid_creds.set("false".into());
                                                            hide_internal_err.set("true".into())
                                                        },
                                                        _ => {
                                                            hide_internal_err.set("false".into());
                                                            hide_invalid_creds.set("true".into());
                                                        }
                                                    }
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

async fn login(email: String, password: String) -> Result<String, String> {
    let mut req_creds = HashMap::new();
    req_creds.insert("email", email);
    req_creds.insert("password", password);
    let mut req_body = HashMap::new();
    req_body.insert("user", req_creds);

    match reqwest::Client::new()
        .post("http://localhost:9091/api/users/login")
        .header(CONTENT_TYPE, "application/json")
        .json(&req_body)
        .send()
        .await
    {
        Ok(res) => match res.status().as_u16() {
            200 => match res.json::<SuccessfulLoginDTO>().await {
                Ok(dto) => Ok(dto.user.token.unwrap()),
                Err(e) => {
                    log::error!(":: SignInPage :: login :: Failed to deserialize response: {}", e);
                    Err("internal_error".into())
                }
            },
            401 => {
                log::warn!(":: SignInPage :: login :: Invalid credentials");
                Err("invalid_credentials".into())
            }
            _ => {
                log::error!(
                    ":: SignInPage :: login :: Unexpected login response status: {} body: {}",
                    res.status(),
                    res.text().await.unwrap_or_default()
                );
                Err("internal_error".into())
            }
        },
        Err(e) => {
            log::error!(":: SignInPage :: login :: Request failed: {}", e);
            Err("internal_error".into())
        }
    }
}
