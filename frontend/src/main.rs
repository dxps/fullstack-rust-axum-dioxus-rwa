#![allow(non_snake_case)]

mod comps;
mod pages;

use crate::comps::{Footer, Header};
use crate::pages::{ArticleAdd, HomePage, SettingsPage, SignInPage, SignUpPage};
use dioxus::prelude::*;
use sir::{global_css, AppStyle};

fn main() {
    // Init debug tool for WebAssembly.
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus::web::launch(App);
}

fn App(cx: Scope) -> Element {
    global_css!(" a:focus { outline: 0; } ");

    cx.render(rsx!(
        AppStyle{ },
        Router {
            Header { }
            Route { to: "/", HomePage {} }
            Route { to: "/signin", SignInPage {} }
            Route { to: "/signup", SignUpPage {} }
            Route { to: "/article_add", ArticleAdd {} }
            Route { to: "/settings", SettingsPage {} }
            Footer{ }
        }
    ))
}
