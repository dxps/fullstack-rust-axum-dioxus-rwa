#![allow(non_snake_case)]

mod comps;
mod pages;

use crate::comps::{Footer, NavBar};
use crate::pages::{HomePage, SignIn, SignUp};
use dioxus::prelude::*;
use sir::{global_css, AppStyle};

fn main() {
    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus::web::launch(App);
}

fn App(cx: Scope) -> Element {
    global_css!(" a:focus { outline: 0; } ");

    cx.render(rsx!(
        AppStyle{ },
        Router {
            NavBar { }
            Route { to: "/", HomePage { }}
            Route { to: "/signin", SignIn { }}
            Route { to: "/signup", SignUp { }}
            Footer{ }
        }
    ))
}
