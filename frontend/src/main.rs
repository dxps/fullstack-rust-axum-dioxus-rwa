#![allow(non_snake_case)]

mod commons;
mod comps;
mod pages;

use crate::commons::{AppState, TOKEN};
use crate::comps::{Footer, Header};
use crate::pages::{
    ArticleAdd, HomePage, NotFoundPage, SettingsPage, SignInPage, SignOutPage, SignUpPage,
};
use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use dioxus_use_storage::use_session_storage;
use sir::{global_css, AppStyle};

fn main() {
    // Init debug tool for WebAssembly.
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    //
    global_css!(" a:focus { outline: 0; } ");

    let mut app_state = AppState::new();
    if let Some(token) = use_session_storage(cx).get(TOKEN) {
        app_state.token = Some(token);
    }
    use_shared_state_provider(cx, || app_state);

    cx.render(rsx!(
        AppStyle{ },
        Router {
            Header { }
            Route { to: "/", HomePage {} }
            Route { to: "/home", HomePage {} }
            Route { to: "/signin", SignInPage {} }
            Route { to: "/signout", SignOutPage {} }
            Route { to: "/signup", SignUpPage {} }
            Route { to: "/article_add", ArticleAdd {} }
            Route { to: "/settings", SettingsPage {} }
            // If the current location doesn't match any of
            // the above routes, render the NotFoundPage component.
            Route { to: "/?", NotFoundPage {} }
            Footer{ }
        }
    ))
}
